use super::*;
use crate::database::model::SecretValue;
use sha1::digest::generic_array::functional::FunctionalSequence;
use sha1::{Digest, Sha1};
use tokio::sync::Semaphore;

/// Indicates the problem with the password.
#[derive(Clone, serde::Serialize)]
pub enum PasswordProblem {
    Common,
    Exposed,
    None,
}

/// Loads the password from the database and checks if it is common or exposed. Uses https://haveibeenpwned.com API. Result is cached in the database.
/// # Error
/// If the content cannot be loaded from the database or if the password cannot be checked.
#[tauri::command]
pub async fn check_password_from_database<'a>(
    id: u64,
    database: State<'a, Database>,
) -> Result<PasswordProblem, &'static str> {
    let content = database
        .get_content(id)
        .map_err(|_| "Failed to load content")?;

    let Value::Password(password) = content.value() else {
        return Err("Content is not a password");
    };

    let password = SecretValue::new(password.to_secret_string());

    check_password(password, database).await
}

/// Semaphore for [`check_password`].
static SEM: Semaphore = Semaphore::const_new(1);

/// Checks if the password is common or exposed. Uses https://haveibeenpwned.com API. Result is cached in the database. Uses a semaphore to prevent multiple requests for the same hash.
/// # Error
/// If semaphore cannot be acquired or if the request fails.
#[tauri::command]
pub async fn check_password<'a>(
    password: SecretValue,
    database: State<'a, Database>,
) -> Result<PasswordProblem, &'static str> {
    if passwords::analyzer::is_common_password(password.expose_secret()) {
        return Ok(PasswordProblem::Common);
    }
    let mut hasher = Sha1::new();
    hasher.update(password.expose_secret().as_bytes());
    let hash: SecretString = SecretString::new(
        hasher
            .finalize()
            .fold(String::with_capacity(40), |mut acc, byte| {
                acc.push_str(&format!("{:02x}", byte).to_uppercase());
                acc
            })
            .into(),
    );
    let semaphore = SEM
        .acquire()
        .await
        .map_err(|_| "Failed to acquire permit")?;
    if let Some(status) = database.get_data_breach_status(hash.expose_secret())? {
        return if status {
            Ok(PasswordProblem::Exposed)
        } else {
            Ok(PasswordProblem::None)
        };
    }
    let (prefix, suffix) = hash.expose_secret().split_at(5);
    let url = SecretString::new(format!("https://api.pwnedpasswords.com/range/{}", prefix).into());
    let response = SecretString::new(
        reqwest::get(url.expose_secret())
            .await
            .map_err(|_| "Failed to get response")?
            .text()
            .await
            .map_err(|_| "Failed to get response text")?
            .into(),
    );
    let result = response
        .expose_secret()
        .lines()
        .any(|line| line.starts_with(suffix));

    drop(semaphore);
    database.add_data_breach_cache(hash.expose_secret(), result)?;

    Ok(if result {
        PasswordProblem::Exposed
    } else {
        PasswordProblem::None
    })
}

/// Returns the strength of the password ([`passwords::scorer::score`])
#[tauri::command]
pub async fn password_strength(password: SecretValue) -> f64 {
    passwords::scorer::score(&passwords::analyzer::analyze(password.expose_secret()))
}

/// Generates a password using [`passwords::PasswordGenerator`].
/// # Error
/// If the password cannot be generated.
#[tauri::command(rename_all = "snake_case")]
pub async fn generate_password<'a>(
    length: usize,
    numbers: bool,
    uppercase_letters: bool,
    lowercase_letters: bool,
    symbols: bool,
) -> Result<SecretValue, &'static str> {
    Ok(SecretValue::new(SecretString::new(
        passwords::PasswordGenerator {
            length,
            numbers,
            lowercase_letters,
            uppercase_letters,
            symbols,
            spaces: false,
            exclude_similar_characters: false,
            strict: true,
        }
        .generate_one()?
        .into(),
    )))
}
