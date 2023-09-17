pub mod password {
    pub use passwords::analyzer::*;
    pub use passwords::scorer::*;
    pub use passwords::*;
    use sha1::{Digest, Sha1};
    use zeroize::Zeroize;
    pub async fn is_exposed<S: AsRef<str>>(
        password: S,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut hasher = Sha1::new();
        hasher.update(password.as_ref().as_bytes());
        let mut hash: String = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
            .to_uppercase();
        let (prefix, suffix) = hash.split_at(5);
        let mut url = format!("https://api.pwnedpasswords.com/range/{}", prefix);
        let mut response = reqwest::get(url.as_str()).await?.text().await?;
        let result = response.lines().any(|line| line.starts_with(suffix));
        url.zeroize();
        hash.zeroize();
        response.zeroize();
        Ok(result)
    }
}

pub mod validate {
    pub use card_validate::Type as card_type;
    pub use card_validate::Validate as card;
    pub use card_validate::ValidateError as card_error;
    use regex::Regex;

    pub fn is_url<S: AsRef<str>>(url: S) -> bool {
        Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(url.as_ref())
    }

    pub fn is_email<S: AsRef<str>>(email: S) -> bool {
        Regex::new(r"([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)") // https://emailregex.com/
            .unwrap()
            .is_match(email.as_ref())
    }

    pub fn is_phone_number<S: AsRef<str>>(phone_number: S) -> bool {
        Regex::new(r"(\+[1-9]{1,4})?[0-9]([0-9]*)")
            .unwrap()
            .is_match(phone_number.as_ref())
    }
}

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}
