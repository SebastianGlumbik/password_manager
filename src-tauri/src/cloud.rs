use crate::database::{Database, DATABASE_FILE_NAME};
use secrecy::{ExposeSecret, SecretString};
use ssh2::Session;
use std::fs::File;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tauri::AppHandle;

pub fn connect(address: &str, username: &str, password: &str) -> Result<Session, &'static str> {
    let mut session = Session::new().map_err(|_| "Failed to initialize session")?;
    session.set_tcp_stream(TcpStream::connect(address).map_err(|_| "Failed to connect")?);
    session.handshake().map_err(|_| "Handshake failed")?;
    session
        .userauth_password(username, password)
        .map_err(|_| "Wrong credentials")?;

    Ok(session)
}

pub fn connect_from_database(database: &Database) -> Result<Session, &'static str> {
    let address = database
        .get_setting("cloud_address")
        .map_err(|_| "Failed to load address")?;
    let username = database
        .get_setting("cloud_username")
        .map_err(|_| "Failed to load username")?;
    let password = database
        .get_setting("cloud_password")
        .map_err(|_| "Failed to load password")?;

    connect(
        address.expose_secret(),
        username.expose_secret(),
        password.expose_secret(),
    )
}

/// Enables cloud sync and returns true if the cloud database already exists.
pub fn enable<'a>(
    address: &str,
    username: &str,
    password: &str,
    app_handle: &AppHandle,
    database: &Database,
) -> Result<bool, &'static str> {
    let address = SecretString::new(
        SocketAddr::from_str(address)
            .or_else(|_| IpAddr::from_str(address).map(|ip| SocketAddr::new(ip, 22)))
            .map_err(|_| "Invalid address")?
            .to_string(),
    );

    let session = connect(address.expose_secret(), username, password)?;
    let sftp = session.sftp().map_err(|_| "Failed to initialize sftp")?;

    database.save_setting("cloud", true.to_string().as_str())?;
    database.save_setting("cloud_address", address.expose_secret())?;
    database.save_setting("cloud_username", username)?;
    database.save_setting("cloud_password", password)?;

    let cloud_path =
        PathBuf::from(app_handle.package_info().name.as_str()).join(DATABASE_FILE_NAME);

    return Ok(sftp.open(cloud_path.as_path()).is_ok());
}

pub fn disable<'a>(database: &Database) -> Result<(), &'static str> {
    database.save_setting("cloud", false.to_string().as_str())?;
    database.delete_setting("cloud_address")?;
    database.delete_setting("cloud_username")?;
    database.delete_setting("cloud_password")?;
    Ok(())
}

pub fn is_enabled(database: &Database) -> bool {
    database
        .get_setting("cloud")
        .map_or(false, |value| value.expose_secret() == "true")
}

/// Returns the last modified time of the cloud database.
pub fn m_time<'a>(app_handle: &AppHandle, database: &Database) -> Result<i64, &'static str> {
    let session = connect_from_database(database)?;
    let sftp = session.sftp().map_err(|_| "Failed to initialize sftp")?;

    let cloud_database_path =
        PathBuf::from(app_handle.package_info().name.as_str()).join(DATABASE_FILE_NAME);

    Ok(sftp
        .stat(cloud_database_path.as_path())
        .map_err(|_| "Failed to get cloud metadata")?
        .mtime
        .ok_or("Failed to get cloud mtime")? as i64)
}

pub fn upload<'a>(app_handle: &AppHandle, database: &Database) -> Result<(), &'static str> {
    let local_database_path = Database::path(app_handle).ok_or("Failed to get database path")?;

    let session = connect_from_database(database)?;
    let sftp = session.sftp().map_err(|_| "Failed to initialize sftp")?;

    let cloud_folder = Path::new(app_handle.package_info().name.as_str());
    if sftp.opendir(cloud_folder).is_err() {
        sftp.mkdir(cloud_folder, 0o755)
            .map_err(|_| "Failed to create folder")?;
    }

    let cloud_database_path = PathBuf::from(cloud_folder).join(DATABASE_FILE_NAME);
    if sftp.open(cloud_database_path.as_path()).is_ok() {
        let backup_path =
            PathBuf::from(cloud_folder).join(format!("{}.backup", DATABASE_FILE_NAME));
        sftp.unlink(backup_path.as_path()).unwrap_or_default();
        sftp.rename(
            cloud_database_path.as_path(),
            backup_path.as_path(),
            Some(ssh2::RenameFlags::all()),
        )
        .map_err(|_| "Failed to create backup")?;
    }

    let mut cloud_database = sftp
        .create(cloud_database_path.as_path())
        .map_err(|_| "Failed to create cloud file")?;

    let mut local_database =
        File::open(local_database_path).map_err(|_| "Failed to open local file")?;

    std::io::copy(&mut local_database, &mut cloud_database).map_err(|_| "Failed to copy file")?;

    Ok(())
}

pub fn download<'a>(app_handle: &AppHandle, database: &Database) -> Result<(), &'static str> {
    let session = connect_from_database(database)?;
    let sftp = session.sftp().map_err(|_| "Failed to initialize sftp")?;

    let cloud_database_path =
        PathBuf::from(app_handle.package_info().name.as_str()).join(DATABASE_FILE_NAME);

    let mut local_database_path =
        Database::path(app_handle).ok_or("Failed to get database path")?;

    let mut cloud_database = sftp
        .open(cloud_database_path.as_path())
        .map_err(|_| "Failed to open cloud file")?;

    let mut backup_path =
        local_database_path.with_file_name(format!("{}.backup", DATABASE_FILE_NAME));

    std::fs::rename(&mut local_database_path, &mut backup_path)
        .map_err(|_| "Failed to create backup")?;

    let mut local_database =
        File::create(local_database_path).map_err(|_| "Failed to create local file")?;

    std::io::copy(&mut cloud_database, &mut local_database).map_err(|_| "Failed to copy file")?;

    Ok(())
}
