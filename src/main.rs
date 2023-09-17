use password_manager::models::*;
use password_manager::utils;
use password_manager::PasswordManager;

#[tokio::main]
async fn main() {
    let mut pm = match PasswordManager::new("password") {
        Ok(pm) => pm,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let mut record = Record::new("Example".to_string(), Category::Login);
    record.add_content(Content::Email(
        specific::Email::new("email".to_string(), true, "example@email.com".to_string()).unwrap(),
    ));
    record.add_content(Content::URL(
        specific::URL::new("website".to_string(), true, "www.example.com".to_string()).unwrap(),
    ));
    let pg = utils::password::PasswordGenerator {
        length: 8,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    record.add_content(Content::Password(specific::Password::new(
        "password".to_string(),
        true,
        pg.generate_one().unwrap(),
    )));
    pm.add_record(record);
    println!("Login Record:");
    for (index, record) in pm.records().iter().enumerate() {
        println!("Record {}: {:?}", index, record);
    }
    pm.save_to_drive().unwrap();
    pm.update_record(0)
        .unwrap()
        .add_content(Content::Text(basic::Text::new(
            "note".to_string(),
            false,
            "This is a note".to_string(),
            basic::TextType::Normal,
        )));
    println!("New note:");
    for (index, record) in pm.records().iter().enumerate() {
        println!("Record {}: {:?}", index, record);
    }
    pm.save_to_drive().unwrap();
    println!("Note with ID");
    for (index, record) in pm.records().iter().enumerate() {
        println!("Record {}: {:?}", index, record);
    }
    pm.clear_records();
    pm.load_from_drive().unwrap();
    println!("From drive:");
    for (index, record) in pm.records().iter().enumerate() {
        println!("Record {}: {:?}", index, record);
    }
    pm.load_content(0).unwrap();
    println!("From drive with content:");
    for (index, record) in pm.records().iter().enumerate() {
        println!("Record {}: {:?}", index, record);
    }
    if let Content::Password(password) = pm.update_record(0).unwrap().update_content(3).unwrap() {
        println!(
            "Exposed password: {}",
            utils::password::is_exposed(password.value()).await.unwrap()
        );
        println!(
            "Password score: {}",
            utils::password::score(&utils::password::analyze(password.value()))
        );
    }
}
