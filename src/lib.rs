use regex::Regex;
use rusqlite::{params, Connection, Result};
use secrecy::{ExposeSecret, Secret};
use std::ops::Not;
use totp_rs::{Rfc6238, TOTP};
use zeroize::{Zeroize, ZeroizeOnDrop};

//TODO split into multiple files
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Login {
    id: u32,
    website: String,
    username: String,
    password: String,
    totp: Option<TOTP>,
    note: Option<String>,
    #[zeroize(skip)]
    last_modified: chrono::DateTime<chrono::Local>,
}

//TODO csv export/import, haveibeeenpwned check

impl Login {
    pub fn new(website: String, username: String, password: String) -> Result<Login, &'static str> {
        if website.trim().is_empty() {
            return Err("Website cannot be empty");
        }
        if username.trim().is_empty() {
            return Err("Username cannot be empty");
        }
        if password.trim().is_empty() {
            return Err("Password cannot be empty");
        }
        if Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(website.as_str()).not(){
            return Err("Invalid URL");
        }

        Ok(Self {
            id: 0,
            website,
            username,
            password,
            totp: None,
            note: None,
            last_modified: chrono::Local::now(),
        })
    }
    pub fn id(&self) -> &u32 {
        &self.id
    }
    pub fn website(&self) -> &str {
        &self.website
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub fn otp_auth_url(&self) -> Secret<Option<String>> {
        Secret::new(self.totp.as_ref().map(|totp| totp.get_url()))
    }
    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
    pub fn last_modified(&self) -> chrono::DateTime<chrono::Local> {
        self.last_modified
    }
    pub fn set_website(&mut self, website: String) -> Result<(), &'static str> {
        if website.trim().is_empty() {
            return Err("Website cannot be empty");
        }
        if Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(website.as_str()).not(){
            return Err("Invalid URL");
        }
        self.website.zeroize();
        self.website = website;
        self.last_modified = chrono::Local::now();
        Ok(())
    }
    pub fn set_username(&mut self, username: String) -> Result<(), &'static str> {
        if username.trim().is_empty() {
            return Err("Username cannot be empty");
        }
        self.username.zeroize();
        self.username = username;
        self.last_modified = chrono::Local::now();
        Ok(())
    }
    pub fn set_password(&mut self, password: String) -> Result<(), &'static str> {
        if password.trim().is_empty() {
            return Err("Password cannot be empty");
        }
        self.password.zeroize();
        self.password = password;
        self.last_modified = chrono::Local::now();
        Ok(())
    }
    pub fn set_totp_url(&mut self, url: &str) -> Result<(), &'static str> {
        let Ok(totp) = TOTP::from_url(url)
            else {
                return Err("Invalid OTP Auth URL")
            };
        self.totp = Some(totp);
        self.last_modified = chrono::Local::now();
        Ok(())
    }
    pub fn set_totp_secret(&mut self, secret: String) -> Result<(), &'static str> {
        let Ok(secret) = totp_rs::Secret::Encoded(secret).to_bytes()
        else {
            return Err("Invalid OTP Secret")
        };
        let Ok(mut rfc6238) = Rfc6238::with_defaults(secret)
            else {
                return Err("Invalid OTP Secret")
            };
        rfc6238.account_name(self.username.clone());
        let Ok(totp) = TOTP::from_rfc6238(rfc6238)
            else {
                return Err("Invalid OTP Secret")
            };
        self.totp = Some(totp);
        self.last_modified = chrono::Local::now();
        Ok(())
    }
    pub fn set_note(&mut self, note: String) {
        self.note.zeroize();
        self.note = Some(note);
        self.last_modified = chrono::Local::now()
    }
    pub fn get_code(&self) -> Result<(String, u64), &'static str> {
        match self.totp {
            Some(ref totp) => {
                if let Ok(code) = totp.generate_current() {
                    if let Ok(time) = totp.ttl() {
                        Ok((code, time))
                    } else {
                        Err("Failed to generate OTP code")
                    }
                } else {
                    Err("Failed to generate OTP code")
                }
            }
            None => Err("No OTP Auth URL"),
        }
    }
}
/*
TODO Credit Card, Note, Contact, etc.

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct CreditCard {
    card_number: String,
    card_holder_name: String,
    expiry_date: String,
    cvv: String,
    note: Option<String>,
    #[zeroize(skip)]
    last_modified: chrono::DateTime<chrono::Local>,
}
*/
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub enum DatabaseItemType {
    Login,
    //CreditCard,
}
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct DatabaseItem {
    title: String,
    subtitle: String,
    id: u32,
    item_type: DatabaseItemType,
}

impl DatabaseItem {
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn subtitle(&self) -> &str {
        &self.subtitle
    }
    pub fn id(&self) -> &u32 {
        &self.id
    }
    pub fn item_type(&self) -> &DatabaseItemType {
        &self.item_type
    }
}

pub struct PasswordManager {
    connection: Connection,
}

impl PasswordManager {
    pub fn new(password: &str) -> Result<PasswordManager, &'static str> {
        if password.trim().is_empty() {
            return Err("Password cannot be empty");
        }
        let connection = Connection::open("database.db").map_err(|_| "Failed to open database")?;
        let mut sql = format!("PRAGMA key = '{password}';");
        connection
            .execute_batch(&sql)
            .map(|_| sql.zeroize())
            .map_err(|_| {
                sql.zeroize();
                "Failed to set a key"
            })?;
        connection
            .execute_batch("SELECT count(*) FROM sqlite_master;")
            .map_err(|_| "Invalid password")?;
        connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS Login (
                    id INTEGER PRIMARY KEY,
                    website TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
                    otp_auth_url TEXT,
                    note TEXT,
                    last_modified DATETIME NOT NULL
                );",
            )
            .map_err(|_| "Failed to create database")?;
        Ok(Self { connection })
    }
    pub fn change_key(&mut self, new_password: &str) -> Result<(), &'static str> {
        let mut sql = format!("PRAGMA rekey = '{new_password}';");
        self.connection
            .execute_batch(&sql)
            .map(|_| sql.zeroize())
            .map_err(|_| {
                sql.zeroize();
                "Failed to set a key"
            })
    }

    pub fn get_items(&self) -> Result<Vec<DatabaseItem>> {
        let sql = "SELECT website, username, id FROM Login;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([], |row| {
            Ok(DatabaseItem {
                title: row.get(0)?,
                subtitle: row.get(1)?,
                id: row.get(2)?,
                item_type: DatabaseItemType::Login,
            })
        })?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }
    pub fn save_login(&self, login: Login) -> Result<()> {
        let website = login.website();
        let username = login.username();
        let password = login.password();
        let otp_auth_url_secret = login.otp_auth_url();
        let otp_auth_url = otp_auth_url_secret.expose_secret();
        let note = login.note();
        let last_modified = login.last_modified();
        let id = login.id();
        let mut params = params![
            website,
            username,
            password,
            otp_auth_url,
            note,
            last_modified,
            id
        ]
        .to_vec();
        let mut sql = if *login.id() == 0 {
            params.pop();
            "INSERT INTO Login (website, username, password, otp_auth_url, note, last_modified) VALUES (?1, ?2, ?3, ?4, ?5, ?6);".to_string()
        } else {
            "UPDATE Login SET website = ?1, username = ?2, password = ?3, otp_auth_url = ?4, note = ?5, last_modified = ?6 WHERE id = ?7;".to_string()
        };
        self.connection
            .execute(&sql, &*params)
            .map(|_| sql.zeroize())
            .map_err(|result| {
                sql.zeroize();
                result
            })
    }
    pub fn get_login(&self, id: u32) -> Result<Login> {
        let sql = "SELECT * FROM Login WHERE id = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        stmt.query_row(params![id], |row| {
            let url = row.get::<_, Option<String>>(4)?;
            let totp = match url {
                Some(mut url) => {
                    if let Ok(totp) = TOTP::from_url(&url) {
                        url.zeroize();
                        Some(totp)
                    } else {
                        url.zeroize();
                        None
                    }
                }
                None => None,
            };
            Ok(Login {
                id: row.get(0)?,
                website: row.get(1)?,
                username: row.get(2)?,
                password: row.get(3)?,
                totp,
                note: row.get(5)?,
                last_modified: row.get(6)?,
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Local};
    fn new_login() -> Login {
        let website = "https://www.example.com".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        Login::new(website, username, password).unwrap()
    }

    #[test]
    fn login_empty_website() {
        let website = "".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_err());
    }
    #[test]
    fn login_invalid_website_1() {
        let website = "example".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_err())
    }
    #[test]
    fn login_invalid_website_2() {
        let website = "example.00".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_err())
    }
    #[test]
    fn login_invalid_website_3() {
        let website = "example.".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_err())
    }
    #[test]
    fn login_invalid_website_4() {
        let website = "www.example".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_err())
    }
    #[test]
    fn login_empty_username() {
        let website = "example.com".to_string();
        let username = "".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_err())
    }
    #[test]
    fn login_empty_password() {
        let website = "example.com".to_string();
        let username = "username".to_string();
        let password = "".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_err())
    }
    #[test]
    fn login_website_1() {
        let website = "example.com".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_ok())
    }
    #[test]
    fn login_website_2() {
        let website = "www.test.example.com".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_ok())
    }
    #[test]
    fn login_website_3() {
        let website = "http://example.com".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_ok())
    }
    #[test]
    fn login_website_4() {
        let website = "https://example.com".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_ok())
    }
    #[test]
    fn login_website_5() {
        let website = "https://www.test.example.com".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_ok())
    }
    #[test]
    fn login_website_6() {
        let website = "https://www.test.example.com/test".to_string();
        let username = "username".to_string();
        let password = "password".to_string();
        let login = Login::new(website, username, password);
        assert!(login.is_ok())
    }
    #[test]
    fn login_get() {
        let time: DateTime<Local> = DateTime::from(
            chrono::DateTime::parse_from_rfc2822("Wed, 18 Feb 2015 23:16:09 GMT").unwrap(),
        );
        let login = Login {
            id: 10,
            website: "https://www.example.com".to_string(),
            username: "username".to_string(),
            password: "password".to_string(),
            totp: None,
            note: None,
            last_modified: time,
        };
        assert_eq!(*login.id(), 10);
        assert_eq!(login.website(), "https://www.example.com");
        assert_eq!(login.username(), "username");
        assert_eq!(login.password(), "password");
        assert_eq!(*login.otp_auth_url().expose_secret(), None);
        assert!(login.get_code().is_err());
        assert_eq!(login.note(), None);
        assert_eq!(login.last_modified(), time);
    }
    #[test]
    fn login_set_empty_website() {
        let mut login = new_login();
        let website = login.website().to_string();
        let result = login.set_website("".to_string());
        assert!(result.is_err());
        assert_eq!(login.website(), website);
    }
    #[test]
    fn login_set_invalid_website_1() {
        let mut login = new_login();
        let website = login.website().to_string();
        let result = login.set_website("example".to_string());
        assert!(result.is_err());
        assert_eq!(login.website(), website);
    }
    #[test]
    fn login_set_invalid_website_2() {
        let mut login = new_login();
        let website = login.website().to_string();
        let result = login.set_website("example.00".to_string());
        assert!(result.is_err());
        assert_eq!(login.website(), website);
    }
    #[test]
    fn login_set_invalid_website_3() {
        let mut login = new_login();
        let website = login.website().to_string();
        let result = login.set_website("example.".to_string());
        assert!(result.is_err());
        assert_eq!(login.website(), website);
    }
    #[test]
    fn login_set_invalid_website_4() {
        let mut login = new_login();
        let website = login.website().to_string();
        let result = login.set_website("www.example".to_string());
        assert!(result.is_err());
        assert_eq!(login.website(), website);
    }
    #[test]
    fn login_set_empty_username() {
        let mut login = new_login();
        let username = login.username().to_string();
        let result = login.set_username("".to_string());
        assert!(result.is_err());
        assert_eq!(login.username(), username);
    }
    #[test]
    fn login_set_empty_password() {
        let mut login = new_login();
        let password = login.password().to_string();
        let result = login.set_password("".to_string());
        assert!(result.is_err());
        assert_eq!(login.password(), password);
    }
    #[test]
    fn login_set_url_1() {
        let mut login = new_login();
        let result = login.set_website("example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(login.website(), "example.com");
    }
    #[test]
    fn login_set_url_2() {
        let mut login = new_login();
        let result = login.set_website("www.test.example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(login.website(), "www.test.example.com");
    }
    #[test]
    fn login_set_url_3() {
        let mut login = new_login();
        let result = login.set_website("http://example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(login.website(), "http://example.com");
    }
    #[test]
    fn login_set_url_4() {
        let mut login = new_login();
        let result = login.set_website("https://example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(login.website(), "https://example.com");
    }
    #[test]
    fn login_set_url_5() {
        let mut login = new_login();
        let result = login.set_website("https://www.test.example.com".to_string());
        assert!(result.is_ok());
        assert_eq!(login.website(), "https://www.test.example.com");
    }
    #[test]
    fn login_set_url_6() {
        let mut login = new_login();
        let result = login.set_website("https://www.test.example.com/test".to_string());
        assert!(result.is_ok());
        assert_eq!(login.website(), "https://www.test.example.com/test");
    }
    #[test]
    fn login_set_get_note_1() {
        let mut login = new_login();
        assert!(login.note().is_none());
        login.set_note("note".to_string());
        assert_eq!(login.note().unwrap(), "note");
    }
    #[test]
    fn login_set_get_note_2() {
        let mut login = new_login();
        assert!(login.note().is_none());
        login.set_note("note".to_string());
        assert_eq!(login.note().unwrap(), "note");
        login.set_note("".to_string());
        assert_eq!(login.note().unwrap(), "");
    }
    #[test]
    fn login_set_invalid_totp_secret_1() {
        let mut login = new_login();
        let url = login.otp_auth_url().expose_secret().clone();
        let result = login.set_totp_secret("".to_string());
        assert!(result.is_err());
        assert_eq!(*login.otp_auth_url().expose_secret(), url);
    }
    #[test]
    fn login_set_invalid_totp_secret_2() {
        let mut login = new_login();
        let url = login.otp_auth_url().expose_secret().clone();
        let result = login.set_totp_secret("secret".to_string());
        assert!(result.is_err());
        assert_eq!(*login.otp_auth_url().expose_secret(), url);
    }
    #[test]
    fn login_set_totp_secret_1() {
        let mut login = new_login();
        let result = login.set_totp_secret("pbopfriytzwfp3jfdiye2s4qoqb5rau5".to_string());
        assert!(result.is_ok());
        assert!(login
            .otp_auth_url()
            .expose_secret()
            .clone()
            .unwrap()
            .to_lowercase()
            .contains("pbopfriytzwfp3jfdiye2s4qoqb5rau5"));
    }
    #[test]
    fn login_set_totp_secret_2() {
        let mut login = new_login();
        let result = login.set_totp_secret("hym6ivjzzjwmmpebxi75uxbwgryhb32y".to_string());
        assert!(result.is_ok());
        assert!(login
            .otp_auth_url()
            .expose_secret()
            .clone()
            .unwrap()
            .to_lowercase()
            .contains("hym6ivjzzjwmmpebxi75uxbwgryhb32y"));
    }
    #[test]
    fn login_set_totp_secret_3() {
        let mut login = new_login();
        let result = login.set_totp_secret("hym6ivjzzjwmmpebxi75uxbwgryhb32y".to_string());
        assert!(result.is_ok());
        let result = login.set_totp_secret("".to_string());
        assert!(result.is_err());
        assert!(login
            .otp_auth_url()
            .expose_secret()
            .as_ref()
            .unwrap()
            .to_lowercase()
            .contains("hym6ivjzzjwmmpebxi75uxbwgryhb32y"));
    }
    #[test]
    fn login_set_invalid_totp_url_1() {
        let mut login = new_login();
        let url = login.otp_auth_url().expose_secret().clone();
        let result = login.set_totp_url("");
        assert!(result.is_err());
        assert_eq!(*login.otp_auth_url().expose_secret(), url);
    }
    #[test]
    fn login_set_invalid_totp_url_2() {
        let mut login = new_login();
        let url = login.otp_auth_url().expose_secret().clone();
        let result =
            login.set_totp_url("otpauth://totp/:secret=HYM6IVJZZJWMMPEBXI75UXBWGRYHB32Y&issuer=");
        assert!(result.is_err());
        assert_eq!(*login.otp_auth_url().expose_secret(), url);
    }
    #[test]
    fn login_set_invalid_totp_url_3() {
        let mut login = new_login();
        let url = login.otp_auth_url().expose_secret().clone();
        let result = login.set_totp_url("otpauth://totp/:username?secret=secret&issuer=");
        assert!(result.is_err());
        assert_eq!(*login.otp_auth_url().expose_secret(), url);
    }
    #[test]
    fn login_set_totp_url_1() {
        let mut login = new_login();
        let result = login.set_totp_url(
            "otpauth://totp/:username?secret=HYM6IVJZZJWMMPEBXI75UXBWGRYHB32Y&issuer=",
        );
        assert!(result.is_ok());
        assert!(login
            .otp_auth_url()
            .expose_secret()
            .as_ref()
            .unwrap()
            .eq("otpauth://totp/:username?secret=HYM6IVJZZJWMMPEBXI75UXBWGRYHB32Y&issuer="));
    }
    #[test]
    fn login_set_totp_url_2() {
        let mut login = new_login();
        let result = login.set_totp_url(
            "otpauth://totp/:username?secret=HYM6IVJZZJWMMPEBXI75UXBWGRYHB32Y&issuer=",
        );
        assert!(result.is_ok());
        let result = login.set_totp_url(
            "otpauth://totp/:username?secret=PBOPFRIYTZWFP3JFDIYE2S4QOQB5RAU5&issuer=",
        );
        assert!(result.is_ok());
        assert!(login
            .otp_auth_url()
            .expose_secret()
            .as_ref()
            .unwrap()
            .eq("otpauth://totp/:username?secret=PBOPFRIYTZWFP3JFDIYE2S4QOQB5RAU5&issuer="));
    }
    //TODO More tests
}
