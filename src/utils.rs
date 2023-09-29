pub mod password {
    pub use passwords::analyzer::*;
    pub use passwords::scorer::*;
    pub use passwords::*;
    use secrecy::{ExposeSecret, SecretString};
    use sha1::{Digest, Sha1};
    pub async fn is_exposed<S: AsRef<str>>(
        password: S,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut hasher = Sha1::new();
        hasher.update(password.as_ref().as_bytes());
        let hash: SecretString = SecretString::new(
            hasher
                .finalize()
                .iter()
                .map(|byte| format!("{:02x}", byte))
                .collect::<String>()
                .to_uppercase(),
        );
        let (prefix, suffix) = hash.expose_secret().split_at(5);
        let url = SecretString::new(format!("https://api.pwnedpasswords.com/range/{}", prefix));
        let response = SecretString::new(reqwest::get(url.expose_secret()).await?.text().await?);
        let result = response
            .expose_secret()
            .lines()
            .any(|line| line.starts_with(suffix));
        Ok(result)
    }
}

pub mod validate {
    pub use card_validate::Type as card_type;
    pub use card_validate::Validate as card;
    pub use card_validate::ValidateError as card_error;
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
