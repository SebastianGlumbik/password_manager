use crate::database::{Database, DATABASE_FILE_NAME};
use ssh2::Session;
use std::fs::File;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Duration;
use tauri::AppHandle;
use tokio::sync::Semaphore;

/// Semaphore for [`CloudManager`].
static SEM: Semaphore = Semaphore::const_new(1);

pub struct CloudManager<'a> {
    session: Session,
    app_handle: &'a AppHandle,
}

impl<'a> CloudManager<'a> {
    fn connect(address: &str, username: &str, password: &str) -> Result<Session, &'static str> {
        let mut session = Session::new().map_err(|_| "Failed to initialize session")?;
        session.set_tcp_stream(
            TcpStream::connect_timeout(
                &SocketAddr::from_str(address)
                    .or_else(|_| IpAddr::from_str(address).map(|ip| SocketAddr::new(ip, 22)))
                    .map_err(|_| "Invalid address")?,
                Duration::from_secs(5),
            )
            .map_err(|_| "Failed to connect")?,
        );
        session.handshake().map_err(|_| "Handshake failed")?;
        session
            .userauth_password(username, password)
            .map_err(|_| "Wrong credentials")?;

        Ok(session)
    }

    /// Connects to the cloud using the credentials from the database.
    pub fn connect_from_database(
        database: &Database,
        app_handle: &'a AppHandle,
    ) -> Result<CloudManager<'a>, &'static str> {
        let address = database
            .get_setting("cloud_address")
            .map_err(|_| "Failed to load address")?;
        let username = database
            .get_setting("cloud_username")
            .map_err(|_| "Failed to load username")?;
        let password = database
            .get_setting("cloud_password")
            .map_err(|_| "Failed to load password")?;

        Ok(CloudManager {
            session: Self::connect(
                address.expose_secret(),
                username.expose_secret(),
                password.expose_secret(),
            )?,
            app_handle,
        })
    }

    /// Enables cloud sync and saves the credentials.
    pub fn enable(
        address: &str,
        username: &str,
        password: &str,
        app_handle: &'a AppHandle,
        database: &Database,
    ) -> Result<CloudManager<'a>, &'static str> {
        let session = Self::connect(address, username, password)?;
        let _ = session.sftp().map_err(|_| "Failed to initialize sftp")?;

        database.save_setting("cloud", true.to_string().as_str())?;
        database.save_setting("cloud_address", address)?;
        database.save_setting("cloud_username", username)?;
        database.save_setting("cloud_password", password)?;

        Ok(CloudManager {
            session,
            app_handle,
        })
    }

    pub fn disable(database: &Database) -> Result<(), &'static str> {
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

    /// Returns true if the cloud database exists.
    pub fn exists(&self) -> Result<bool, &'static str> {
        let sftp = self
            .session
            .sftp()
            .map_err(|_| "Failed to initialize sftp")?;
        let cloud_path =
            PathBuf::from(self.app_handle.package_info().name.as_str()).join(DATABASE_FILE_NAME);
        Ok(sftp.open(cloud_path.as_path()).is_ok())
    }

    /// Returns the last modified time of the cloud database.
    pub fn m_time(&self) -> Result<i64, &'static str> {
        let sftp = self
            .session
            .sftp()
            .map_err(|_| "Failed to initialize sftp")?;

        let cloud_database_path =
            PathBuf::from(self.app_handle.package_info().name.as_str()).join(DATABASE_FILE_NAME);

        Ok(sftp
            .stat(cloud_database_path.as_path())
            .map_err(|_| "Failed to get cloud metadata")?
            .mtime
            .ok_or("Failed to get cloud mtime")? as i64)
    }

    pub async fn upload(&self) -> Result<(), &'static str> {
        let local_database_path =
            Database::path(self.app_handle).ok_or("Failed to get database path")?;

        let sftp = self
            .session
            .sftp()
            .map_err(|_| "Failed to initialize sftp")?;

        let cloud_folder = Path::new(self.app_handle.package_info().name.as_str());

        let semaphore = SEM
            .acquire()
            .await
            .map_err(|_| "Failed to acquire permit")?;

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

        std::io::copy(&mut local_database, &mut cloud_database)
            .map_err(|_| "Failed to copy file")?;

        drop(semaphore);

        Ok(())
    }

    pub async fn download(&self) -> Result<(), &'static str> {
        let sftp = self
            .session
            .sftp()
            .map_err(|_| "Failed to initialize sftp")?;

        let cloud_database_path =
            PathBuf::from(self.app_handle.package_info().name.as_str()).join(DATABASE_FILE_NAME);

        let mut local_database_path =
            Database::path(self.app_handle).ok_or("Failed to get database path")?;

        let semaphore = SEM
            .acquire()
            .await
            .map_err(|_| "Failed to acquire permit")?;

        let mut cloud_database = sftp
            .open(cloud_database_path.as_path())
            .map_err(|_| "Failed to open cloud file")?;

        let mut backup_path =
            local_database_path.with_file_name(format!("{}.backup", DATABASE_FILE_NAME));

        std::fs::rename(&mut local_database_path, &mut backup_path)
            .map_err(|_| "Failed to create backup")?;

        let mut local_database =
            File::create(local_database_path).map_err(|_| "Failed to create local file")?;

        std::io::copy(&mut cloud_database, &mut local_database)
            .map_err(|_| "Failed to copy file")?;

        drop(semaphore);

        Ok(())
    }
}
