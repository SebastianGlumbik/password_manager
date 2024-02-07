pub mod password {
    use secrecy::{ExposeSecret, SecretString};
    use sha1::digest::generic_array::functional::FunctionalSequence;
    use sha1::{Digest, Sha1};

    pub async fn is_exposed<S: AsRef<str>>(
        password: S,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut hasher = Sha1::new();
        hasher.update(password.as_ref().as_bytes());
        let hash: SecretString = SecretString::new(hasher.finalize().fold(
            String::with_capacity(40),
            |mut acc, byte| {
                acc.push_str(&format!("{:02x}", byte).to_uppercase());
                acc
            },
        ));
        let (prefix, suffix) = hash.expose_secret().split_at(5);
        let url = SecretString::new(format!("https://api.pwnedpasswords.com/range/{}", prefix));
        let response = SecretString::new(reqwest::get(url.expose_secret()).await?.text().await?);
        Ok(response
            .expose_secret()
            .lines()
            .any(|line| line.starts_with(suffix)))
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
