use card_validate::Validate;
use passwords::analyzer;
use passwords::scorer;
use regex::Regex;
use rusqlite::{params, Connection, Result, Row};
use secrecy::{ExposeSecret, SecretString};
use sha1::{Digest, Sha1};
use std::ops::Not;
use totp_rs::{Rfc6238, TOTP as TOTP_RS};
use zeroize::{Zeroize, ZeroizeOnDrop};

//TODO split into multiple files
//TODO Comments

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Integer {
    id: u32,
    label: String,
    integer: i64,
}

impl Integer {
    pub fn new(label: String, integer: i64) -> Self {
        Integer::new_set_id(0, label, integer)
    }
    pub fn new_set_id(id: u32, label: String, integer: i64) -> Self {
        Self { id, label, integer }
    }
    pub fn integer(&self) -> &i64 {
        &self.integer
    }
    pub fn set_integer(&mut self, integer: i64) {
        self.integer.zeroize();
        self.integer = integer;
    }
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Real {
    id: u32,
    label: String,
    real: f64,
}

impl Real {
    pub fn new(label: String, real: f64) -> Self {
        Real::new_set_id(0, label, real)
    }
    pub fn new_set_id(id: u32, label: String, real: f64) -> Self {
        Self { id, label, real }
    }
    pub fn real(&self) -> &f64 {
        &self.real
    }
    pub fn set_real(&mut self, real: f64) {
        self.real.zeroize();
        self.real = real;
    }
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Text {
    id: u32,
    label: String,
    text: String,
    sensitive: bool,
}

impl Text {
    pub fn new(label: String, text: String, sensitive: bool) -> Self {
        Text::new_set_id(0, label, text, sensitive)
    }
    pub fn new_set_id(id: u32, label: String, text: String, sensitive: bool) -> Self {
        Self {
            id,
            label,
            text,
            sensitive,
        }
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn sensitive(&self) -> &bool {
        &self.sensitive
    }
    pub fn set_text(&mut self, text: String) {
        self.text.zeroize();
        self.text = text;
    }
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Datetime {
    id: u32,
    label: String,
    #[zeroize(skip)]
    datetime: chrono::DateTime<chrono::Local>,
}

impl Datetime {
    pub fn new(label: String, datetime: chrono::DateTime<chrono::Local>) -> Self {
        Datetime::new_set_id(0, label, datetime)
    }
    pub fn new_set_id(id: u32, label: String, datetime: chrono::DateTime<chrono::Local>) -> Self {
        Self {
            id,
            label,
            datetime,
        }
    }
    pub fn datetime(&self) -> &chrono::DateTime<chrono::Local> {
        &self.datetime
    }
    pub fn set_datetime(&mut self, datetime: chrono::DateTime<chrono::Local>) {
        self.datetime = datetime;
    }
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Password {
    id: u32,
    label: String,
    password: String,
}

impl Password {
    pub fn new(label: String, password: String) -> Self {
        Password::new_set_id(0, label, password)
    }
    pub fn new_set_id(id: u32, label: String, password: String) -> Self {
        Self {
            id,
            label,
            password,
        }
    }
    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn score(&self) -> f64 {
        scorer::score(&analyzer::analyze(&self.password))
    }

    pub fn is_common(&self) -> bool {
        analyzer::analyze(&self.password).is_common()
    }

    pub async fn exposed(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut hasher = Sha1::new();
        hasher.update(self.password.as_bytes());
        let hash = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>()
            .to_uppercase();
        let (prefix, suffix) = hash.split_at(5);
        let url = format!("https://api.pwnedpasswords.com/range/{}", prefix);
        let response = reqwest::get(url.as_str()).await?.text().await?;
        Ok(response.lines().any(|line| line.starts_with(suffix)))
    }
    pub fn analyze(&self) -> analyzer::AnalyzedPassword {
        analyzer::analyze(&self.password)
    }
    pub fn set_password(&mut self, password: String) {
        self.password.zeroize();
        self.password = password;
    }
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Login {
    id: u32,
    website: String,
    username: String,
    password: Password,
}

impl Login {
    pub fn new(website: String, username: String, password: String) -> Result<Login, &'static str> {
        Login::new_set_id(
            0,
            website,
            username,
            Password::new("Password".to_string(), password),
        )
    }
    pub fn new_set_id(
        mut id: u32,
        mut website: String,
        mut username: String,
        mut password: Password,
    ) -> Result<Login, &'static str> {
        if Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(website.as_str()).not(){
            id.zeroize();
            website.zeroize();
            username.zeroize();
            password.zeroize();
            return Err("Invalid URL");
        }
        Ok(Self {
            id,
            website,
            username,
            password,
        })
    }
    pub fn website(&self) -> &str {
        &self.website
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &Password {
        &self.password
    }
    pub fn set_website(&mut self, mut website: String) -> Result<(), &'static str> {
        if Regex::new(r"(http(s)?://.)?(www\.)?[-a-zA-Z0-9@:%._+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_+.~#?&/=]*)").unwrap().is_match(website.as_str()).not(){
            website.zeroize();
            return Err("Invalid URL");
        }
        self.website.zeroize();
        self.website = website;
        Ok(())
    }
    pub fn set_username(&mut self, username: String) {
        self.username.zeroize();
        self.username = username;
    }
    pub fn set_password(&mut self, password: String) {
        self.password.set_password(password)
    }
}
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct TOTP {
    id: u32,
    label: String,
    totp: TOTP_RS,
}

impl TOTP {
    pub fn new_from_url(label: String, url: String) -> Result<TOTP, &'static str> {
        TOTP::new_from_url_set_id(0, label, url)
    }
    pub fn new_from_url_set_id(
        mut id: u32,
        mut label: String,
        mut url: String,
    ) -> Result<TOTP, &'static str> {
        let Ok(totp) = TOTP_RS::from_url(&url) else {
            id.zeroize();
            label.zeroize();
            url.zeroize();
            return Err("Invalid OTP Auth URL");
        };
        url.zeroize();
        Ok(Self { id, label, totp })
    }
    pub fn new_from_secret(mut label: String, secret: String) -> Result<TOTP, &'static str> {
        let Ok(secret) = totp_rs::Secret::Encoded(secret).to_bytes() else {
            label.zeroize();
            return Err("Invalid OTP Secret");
        };
        let Ok(mut rfc6238) = Rfc6238::with_defaults(secret) else {
            label.zeroize();
            return Err("Invalid OTP Secret");
        };
        rfc6238.account_name(label.clone());
        let Ok(totp) = TOTP_RS::from_rfc6238(rfc6238) else {
            label.zeroize();
            return Err("Invalid OTP Secret");
        };
        Ok(Self { id: 0, label, totp })
    }
    pub fn get_code(&self) -> Result<(String, u64), &'static str> {
        if let Ok(code) = self.totp.generate_current() {
            if let Ok(time) = self.totp.ttl() {
                Ok((code, time))
            } else {
                Err("Failed to generate remaining time")
            }
        } else {
            Err("Failed to generate OTP code")
        }
    }
    pub fn get_url(&self) -> SecretString {
        SecretString::new(self.totp.get_url())
    }
}
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct CreditCard {
    id: u32,
    card_number: String,
    holder_name: String,
    expiry_date: String,
    security_code: String,
}
impl CreditCard {
    pub fn new(
        card_number: String,
        holder_name: String,
        expiry_date: String,
        security_code: String,
    ) -> Result<CreditCard, &'static str> {
        CreditCard::new_set_id(0, card_number, holder_name, expiry_date, security_code)
    }
    pub fn new_set_id(
        mut id: u32,
        mut card_number: String,
        mut holder_name: String,
        mut expiry_date: String,
        mut security_code: String,
    ) -> Result<CreditCard, &'static str> {
        if let Err(e) = Validate::from(&card_number) {
            id.zeroize();
            card_number.zeroize();
            holder_name.zeroize();
            expiry_date.zeroize();
            security_code.zeroize();
            return match e {
                card_validate::ValidateError::InvalidFormat => Err("Invalid card number"),
                card_validate::ValidateError::InvalidLength => Err("Invalid card length"),
                _ => Err("Unknown card"),
            };
        }
        if Regex::new(r"(0[1-9]|1[0-2])/[0-9]{2}")
            .unwrap()
            .is_match(expiry_date.as_str())
            .not()
        {
            id.zeroize();
            card_number.zeroize();
            holder_name.zeroize();
            expiry_date.zeroize();
            security_code.zeroize();
            return Err("Invalid expiry date");
        }
        if Regex::new(r"[0-9]{3,4}")
            .unwrap()
            .is_match(security_code.as_str())
            .not()
        {
            id.zeroize();
            card_number.zeroize();
            holder_name.zeroize();
            expiry_date.zeroize();
            security_code.zeroize();
            return Err("Invalid security code");
        }
        Ok(Self {
            id,
            card_number,
            holder_name,
            expiry_date,
            security_code,
        })
    }
    pub fn card_number(&self) -> &str {
        &self.card_number
    }
    pub fn holder_name(&self) -> &str {
        &self.holder_name
    }
    pub fn expiry_date(&self) -> &str {
        &self.expiry_date
    }
    pub fn security_code(&self) -> &str {
        &self.security_code
    }
    pub fn card_type(&self) -> String {
        Validate::from(&self.card_number).unwrap().card_type.name()
    }
    pub fn set_card_number(&mut self, mut card_number: String) -> Result<(), &'static str> {
        if let Err(e) = Validate::from(&card_number) {
            card_number.zeroize();
            return match e {
                card_validate::ValidateError::InvalidFormat => Err("Invalid card number"),
                card_validate::ValidateError::InvalidLength => Err("Invalid card length"),
                _ => Err("Unknown error"),
            };
        }
        self.card_number.zeroize();
        self.card_number = card_number;
        Ok(())
    }
    pub fn set_holder_name(&mut self, holder_name: String) {
        self.holder_name.zeroize();
        self.holder_name = holder_name;
    }
    pub fn set_expiry_date(&mut self, mut expiry_date: String) -> Result<(), &'static str> {
        if Regex::new(r"(0[1-9]|1[0-2])/[0-9]{2}")
            .unwrap()
            .is_match(expiry_date.as_str())
            .not()
        {
            expiry_date.zeroize();
            return Err("Invalid expiry date");
        }
        self.expiry_date.zeroize();
        self.expiry_date = expiry_date;
        Ok(())
    }
    pub fn set_security_code(&mut self, mut security_code: String) -> Result<(), &'static str> {
        if Regex::new(r"[0-9]{3,4}")
            .unwrap()
            .is_match(security_code.as_str())
            .not()
        {
            security_code.zeroize();
            return Err("Invalid security code");
        }
        self.security_code.zeroize();
        self.security_code = security_code;
        Ok(())
    }
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Note {
    id: u32,
    subject: String,
    note: String,
}

impl Note {
    pub fn new(subject: String, note: String) -> Self {
        Note::new_set_id(0, subject, note)
    }
    pub fn new_set_id(id: u32, subject: String, note: String) -> Self {
        Self { id, subject, note }
    }
    pub fn subject(&self) -> &str {
        &self.subject
    }
    pub fn set_subject(&mut self, subject: String) {
        self.subject.zeroize();
        self.subject = subject;
    }
    pub fn note(&self) -> &str {
        &self.note
    }
    pub fn set_note(&mut self, note: String) {
        self.note.zeroize();
        self.note = note;
    }
}
//TODO More MainItems, AdditionalItems
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Custom {
    id: u32,
    name: String,
}

impl Custom {
    pub fn new(name: String) -> Self {
        Custom::new_set_id(0, name)
    }
    pub fn new_set_id(id: u32, name: String) -> Self {
        Self { id, name }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name.zeroize();
        self.name = name;
    }
}
#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub enum Category {
    Login,
    CreditCard,
    Note,
    Custom(Custom),
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub enum MainItem {
    Login(Login),
    CreditCard(CreditCard),
    Note(Note),
    None,
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub enum AdditionalItem {
    Integer(Integer),
    Real(Real),
    Text(Text),
    Datetime(Datetime),
    Password(Password),
    TOTP(TOTP),
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct RecordContent {
    pub main: MainItem,
    pub additional: Vec<AdditionalItem>,
}

#[derive(Debug, Zeroize, ZeroizeOnDrop)]
pub struct Record {
    id: u32,
    name: String,
    #[zeroize(skip)]
    created: chrono::DateTime<chrono::Local>,
    #[zeroize(skip)]
    last_modified: chrono::DateTime<chrono::Local>,
    category: Category,
    content: Option<RecordContent>,
}
impl Record {
    pub fn new(name: String, category: Category) -> Record {
        Record::new_set_id(
            0,
            name,
            chrono::Local::now(),
            chrono::Local::now(),
            category,
        )
    }
    pub fn new_set_id(
        id: u32,
        name: String,
        created: chrono::DateTime<chrono::Local>,
        last_modified: chrono::DateTime<chrono::Local>,
        category: Category,
    ) -> Record {
        Self {
            id,
            name,
            created,
            last_modified,
            category,
            content: None,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn created(&self) -> chrono::DateTime<chrono::Local> {
        self.created
    }
    pub fn last_modified(&self) -> chrono::DateTime<chrono::Local> {
        self.last_modified
    }
    pub fn category(&self) -> &Category {
        &self.category
    }
    pub fn content(&self) -> Option<&RecordContent> {
        match &self.content {
            Some(content) => Some(content),
            None => None,
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name.zeroize();
        self.name = name;
    }
    pub fn set_content(&mut self, content: RecordContent) {
        self.content = Some(content);
    }
    pub fn edit_content(&mut self) -> Option<&mut RecordContent> {
        match &mut self.content {
            Some(content) => Some(content),
            None => None,
        }
    }
    pub fn delete_content(&mut self) {
        self.content = None;
    }
}

pub trait Id {
    fn id(&self) -> &u32;
    fn set_id(&mut self, id: u32);
}

macro_rules! impl_id {
    (for $($t:ty),+) => {
        $(impl Id for $t {
            fn id(&self) -> &u32 {
                &self.id
            }
            fn set_id(&mut self, id: u32) {
                self.id.zeroize();
                self.id = id;
            }
        })*
    }
}

impl_id!(for Custom, Record, Integer, Real, Text, Datetime, Login, Password, TOTP, CreditCard, Note);

pub trait Label {
    fn label(&self) -> &str;
    fn set_label(&mut self, label: String);
}

macro_rules! impl_label {
    (for $($t:ty),+) => {
        $(impl Label for $t {
            fn label(&self) -> &str {
                &self.label
            }
            fn set_label(&mut self, label: String) {
                self.label.zeroize();
                self.label = label;
            }
        })*
    }
}

impl_label!(for Integer, Real, Text, Datetime, Password, TOTP);

//TODO Delete from database, csv export/import, cloud backup

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
                "create table if not exists Category (
                        id_category integer primary key,
                        name text not null
                    );
                    insert into Category (name) values ('Login');
                    insert into Category (name) values ('Credit Card');
                    insert into Category (name) values ('Note');
                    create table if not exists Record (
                        id_record integer primary key,
                        name text not null,
                        created datetime not null,
                        last_modified datetime not null,
                        id_category integer not null,
                        foreign key (id_category) references Category(id_category) on update cascade on delete restrict
                    );
                    create table if not exists 'Integer' (
                      id_integer integer primary key,
                      label text not null,
                      integer integer not null,
                      id_record integer not null,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists Real (
                      id_real integer primary key,
                      label text not null,
                      real integer not null,
                      id_record integer not null,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists Text (
                      id_text integer primary key,
                      label text not null,
                      text text not null,
                      sensitive integer not null,
                      id_record integer not null,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists Datetime (
                      id_datetime integer primary key,
                      label text not null,
                      datetime datetime not null,
                      id_record integer not null,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists Password (
                      id_password integer primary key,
                      label text not null,
                      password text not null,
                      id_record integer,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists Login (
                        id_login integer primary key,
                        website text not null,
                        username text not null,
                        id_password integer not null,
                        id_record integer not null,
                        foreign key (id_password) references Password(id_password) on update cascade on delete cascade,
                        foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists TOTP(
                      id_totp integer primary key,
                      label text not null,
                      otp_auth text not null,
                      id_record integer not null,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists CreditCard(
                      id_credit_card integer primary key,
                      card_number text not null,
                      holder_name text not null,
                      expiry_date text not null,
                      security_code text not null,
                      id_record integer not null,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );
                    create table if not exists Note(
                      id_note integer primary key,
                      subject text not null,
                      note text not null,
                      id_record integer not null,
                      foreign key (id_record) references Record(id_record) on update cascade on delete cascade
                    );",
            ).map_err(|_| "Failed to create database")?;
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

    pub fn get_category(&self, id: u32) -> Result<Category> {
        Ok(match id {
            1 => Category::Login,
            2 => Category::CreditCard,
            3 => Category::Note,
            id_category => {
                let sql = "SELECT id_category, name FROM Category WHERE id_category = ?1;";
                let mut stmt = self.connection.prepare(sql)?;
                stmt.query_row(params![id_category], |row| {
                    Ok(Category::Custom(Custom::new_set_id(
                        row.get(0)?,
                        row.get(1)?,
                    )))
                })?
            }
        })
    }

    fn row_to_record(&self, row: &Row) -> Result<Record> {
        Ok(Record::new_set_id(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            self.get_category(row.get(4)?)?,
        ))
    }
    pub fn get_record(&self, id: u32) -> Result<Record> {
        let sql =
            "SELECT id_record, name, created, last_modified, id_category FROM Record WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        stmt.query_row(params![id], |row| self.row_to_record(row))
    }
    pub fn get_all_records(&self) -> Result<Vec<Record>> {
        let sql = "SELECT id_record, name, created, last_modified, id_category FROM Record;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([], |row| self.row_to_record(row))?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }
    pub fn get_password(&self, id: u32) -> Result<Password> {
        let sql = "SELECT id_password, label, password FROM Password WHERE id_password = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        stmt.query_row(params![id], |row| {
            Ok(Password::new_set_id(row.get(0)?, row.get(1)?, row.get(2)?))
        })
    }
    fn get_login_for_record(&self, id_record: &u32) -> Result<Login> {
        let sql =
            "SELECT id_login, website, username, id_password FROM Login WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        stmt.query_row(params![id_record], |row| {
            Ok(Login::new_set_id(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                self.get_password(row.get(3)?)?,
            )
            .unwrap())
        })
    }
    fn get_credit_card_for_record(&self, id_record: &u32) -> Result<CreditCard> {
        let sql = "SELECT id_credit_card, card_number, holder_name, expiry_date, security_code FROM CreditCard WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        stmt.query_row(params![id_record], |row| {
            Ok(CreditCard::new_set_id(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
            )
            .unwrap())
        })
    }

    fn get_note_for_record(&self, id_record: &u32) -> Result<Note> {
        let sql = "SELECT id_note, subject, note FROM Note WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        stmt.query_row(params![id_record], |row| {
            Ok(Note::new_set_id(row.get(0)?, row.get(1)?, row.get(2)?))
        })
    }

    fn get_integer_for_record(&self, id_record: &u32) -> Result<Vec<AdditionalItem>> {
        let sql = "SELECT id_integer, label, integer FROM Integer WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([id_record], |row| {
            Ok(AdditionalItem::Integer(Integer::new_set_id(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            )))
        })?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }
    fn get_real_for_record(&self, id_record: &u32) -> Result<Vec<AdditionalItem>> {
        let sql = "SELECT id_real, label, real FROM Real WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([id_record], |row| {
            Ok(AdditionalItem::Real(Real::new_set_id(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            )))
        })?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }

    fn get_text_for_record(&self, id_record: &u32) -> Result<Vec<AdditionalItem>> {
        let sql = "SELECT id_text, label, text, sensitive FROM Text WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([id_record], |row| {
            Ok(AdditionalItem::Text(Text::new_set_id(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            )))
        })?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }
    fn get_datetime_for_record(&self, id_record: &u32) -> Result<Vec<AdditionalItem>> {
        let sql = "SELECT id_datetime, label, datetime FROM Datetime WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([id_record], |row| {
            Ok(AdditionalItem::Datetime(Datetime::new_set_id(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            )))
        })?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }

    fn get_password_for_record(&self, id_record: &u32) -> Result<Vec<AdditionalItem>> {
        let sql = "SELECT id_password, label, password FROM Password WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([id_record], |row| {
            Ok(AdditionalItem::Password(Password::new_set_id(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            )))
        })?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }

    fn get_totp_for_record(&self, id_record: &u32) -> Result<Vec<AdditionalItem>> {
        let sql = "SELECT id_totp, label, otp_auth FROM TOTP WHERE id_record = ?1;";
        let mut stmt = self.connection.prepare(sql)?;
        let items_iter = stmt.query_map([id_record], |row| {
            Ok(AdditionalItem::TOTP(
                TOTP::new_from_url_set_id(row.get(0)?, row.get(1)?, row.get(2)?).unwrap(),
            ))
        })?;
        Ok(items_iter.map(|item| item.unwrap()).collect())
    }
    pub fn load_record_content(&self, record: &mut Record) -> Result<()> {
        let main = match record.category() {
            Category::Login => MainItem::Login(self.get_login_for_record(record.id())?),
            Category::CreditCard => {
                MainItem::CreditCard(self.get_credit_card_for_record(record.id())?)
            }
            Category::Note => MainItem::Note(self.get_note_for_record(record.id())?),
            Category::Custom(_) => MainItem::None,
        };
        let mut additional: Vec<AdditionalItem> = Vec::new();
        additional.append(&mut self.get_integer_for_record(record.id())?); //Integer
        additional.append(&mut self.get_real_for_record(record.id())?); //Real
        additional.append(&mut self.get_text_for_record(record.id())?); //Text
        additional.append(&mut self.get_datetime_for_record(record.id())?); //Datetime
        additional.append(&mut self.get_password_for_record(record.id())?); //Password
        additional.append(&mut self.get_totp_for_record(record.id())?); //TOTP
        record.set_content(RecordContent { main, additional });
        Ok(())
    }
    pub fn save_record(&mut self, record: Record) -> Result<()> {
        let transaction = self.connection.transaction()?;
        let id_category = match record.category() {
            Category::Login => 1,
            Category::CreditCard => 2,
            Category::Note => 3,
            Category::Custom(custom) => match custom.id() {
                0 => {
                    let sql = "INSERT INTO Category (name) VALUES (?1);";
                    transaction.execute(sql, params![custom.name()])?;
                    transaction.last_insert_rowid() as u32
                }
                _ => {
                    let sql = "UPDATE Category SET name = ?1 WHERE id_category = ?2;";
                    transaction.execute(sql, params![custom.name(), custom.id()])?;
                    *custom.id()
                }
            },
        };
        let name = record.name();
        let created = record.created();
        let last_modified = record.last_modified();
        let id = record.id();
        let mut params = params![name, created, last_modified, id_category, id].to_vec();
        let sql = if *record.id() == 0 {
            params.pop();
            "INSERT INTO Record (name, created, last_modified, id_category) VALUES (?1, ?2, ?3, ?4);"
        } else {
            "UPDATE Record SET name = ?1, created = ?2, last_modified = ?3, id_category = ?4 WHERE id_record = ?5;"
        };
        transaction.execute(sql, &*params)?;
        let id_record = transaction.last_insert_rowid();
        if let Some(content) = record.content() {
            match &content.main {
                MainItem::Login(item) => {
                    let website = item.website();
                    let username = item.username();
                    let id_password = {
                        let label = item.password().label();
                        let password = item.password().password();
                        let id = item.password().id();
                        let mut params = params![label, password, id].to_vec();
                        let sql = if *id == 0 {
                            params.pop();
                            "INSERT INTO Password (label, password) VALUES (?1, ?2);"
                        } else {
                            "UPDATE Password SET label = ?1, password = ?2 WHERE id_password = ?3;"
                        };
                        transaction.execute(sql, &*params)?;
                        transaction.last_insert_rowid()
                    };
                    let id = item.id();
                    let mut params = params![website, username, id_password].to_vec();
                    let sql = if *id == 0 {
                        params.append(&mut params![id_record].to_vec());
                        "INSERT INTO Login (website, username, id_password, id_record) VALUES (?1, ?2, ?3, ?4);"
                    } else {
                        params.append(&mut params![id].to_vec());
                        "UPDATE Login SET website = ?1, username = ?2, id_password = ?3 WHERE id_login = ?4;"
                    };
                    transaction.execute(sql, &*params)?;
                }
                MainItem::CreditCard(item) => {
                    let card_number = item.card_number();
                    let holder_name = item.holder_name();
                    let expiry_date = item.expiry_date();
                    let security_code = item.security_code();
                    let id = item.id();
                    let mut params =
                        params![card_number, holder_name, expiry_date, security_code].to_vec();
                    let sql = if *id == 0 {
                        params.append(&mut params![id_record].to_vec());
                        "INSERT INTO CreditCard (card_number, holder_name, expiry_date, security_code, id_record) VALUES (?1, ?2, ?3, ?4, ?5);"
                    } else {
                        params.append(&mut params![id].to_vec());
                        "UPDATE CreditCard SET card_number = ?1, holder_name = ?2, expiry_date = ?3, security_code = ?4 WHERE id_credit_card = ?5;"
                    };
                    transaction.execute(sql, &*params)?;
                }
                MainItem::Note(item) => {
                    let subject = item.subject();
                    let note = item.note();
                    let id = item.id();
                    let mut params = params![subject, note].to_vec();
                    let sql = if *id == 0 {
                        params.append(&mut params![id_record].to_vec());
                        "INSERT INTO Note (subject, note, id_record) VALUES (?1, ?2, ?3);"
                    } else {
                        params.append(&mut params![id].to_vec());
                        "UPDATE Note SET subject = ?1, note = ?2 WHERE id_note = ?3;"
                    };
                    transaction.execute(sql, &*params)?;
                }
                MainItem::None => {}
            };
            for additional_item in content.additional.iter() {
                match additional_item {
                    AdditionalItem::Integer(item) => {
                        let label = item.label();
                        let integer = item.integer();
                        let id = item.id();
                        let mut params = params![label, integer].to_vec();
                        let sql = if *id == 0 {
                            params.append(&mut params![id_record].to_vec());
                            "INSERT INTO Integer (label, integer, id_record) VALUES (?1, ?2, ?3);"
                        } else {
                            params.append(&mut params![id].to_vec());
                            "UPDATE Integer SET label = ?1, integer = ?2 WHERE id_integer = ?3;"
                        };
                        transaction.execute(sql, &*params)?;
                    }
                    AdditionalItem::Real(item) => {
                        let label = item.label();
                        let real = item.real();
                        let id = item.id();
                        let mut params = params![label, real].to_vec();
                        let sql = if *id == 0 {
                            params.append(&mut params![id_record].to_vec());
                            "INSERT INTO Real (label, real, id_record) VALUES (?1, ?2, ?3);"
                        } else {
                            params.append(&mut params![id].to_vec());
                            "UPDATE Real SET label = ?1, real = ?2 WHERE id_real = ?3;"
                        };
                        transaction.execute(sql, &*params)?;
                    }
                    AdditionalItem::Text(item) => {
                        let label = item.label();
                        let text = item.text();
                        let sensitive = item.sensitive();
                        let id = item.id();
                        let mut params = params![label, text, sensitive].to_vec();
                        let sql = if *id == 0 {
                            params.append(&mut params![id_record].to_vec());
                            "INSERT INTO Text (label, text, sensitive, id_record) VALUES (?1, ?2, ?3, ?4);"
                        } else {
                            params.append(&mut params![id].to_vec());
                            "UPDATE Text SET label = ?1, text = ?2, sensitive = ?3 WHERE id_text = ?4;"
                        };
                        transaction.execute(sql, &*params)?;
                    }
                    AdditionalItem::Datetime(item) => {
                        let label = item.label();
                        let datetime = item.datetime();
                        let id = item.id();
                        let mut params = params![label, datetime].to_vec();
                        let sql = if *id == 0 {
                            params.append(&mut params![id_record].to_vec());
                            "INSERT INTO Datetime (label, datetime, id_record) VALUES (?1, ?2, ?3);"
                        } else {
                            params.append(&mut params![id].to_vec());
                            "UPDATE Datetime SET label = ?1, datetime = ?2 WHERE id_datetime = ?3;"
                        };
                        transaction.execute(sql, &*params)?;
                    }
                    AdditionalItem::Password(item) => {
                        let label = item.label();
                        let password = item.password();
                        let id = item.id();
                        let mut params = params![label, password].to_vec();
                        let sql = if *id == 0 {
                            params.append(&mut params![id_record].to_vec());
                            "INSERT INTO Password (label, password, id_record) VALUES (?1, ?2, ?3);"
                        } else {
                            params.append(&mut params![id].to_vec());
                            "UPDATE Password SET label = ?1, password = ?2 WHERE id_password = ?3;"
                        };
                        transaction.execute(sql, &*params)?;
                    }
                    AdditionalItem::TOTP(item) => {
                        let label = item.label();
                        let otp_auth_secret = item.get_url();
                        let otp_auth = otp_auth_secret.expose_secret();
                        let id = item.id();
                        let mut params = params![label, otp_auth].to_vec();
                        let sql = if *id == 0 {
                            params.append(&mut params![id_record].to_vec());
                            "INSERT INTO TOTP (label, otp_auth, id_record) VALUES (?1, ?2, ?3);"
                        } else {
                            params.append(&mut params![id].to_vec());
                            "UPDATE TOTP SET label = ?1, otp_auth = ?2 WHERE id_totp = ?3;"
                        };
                        transaction.execute(sql, &*params)?;
                    }
                }
            }
        }
        transaction.commit()?;
        Ok(())
    }
}
//TODO Tests
