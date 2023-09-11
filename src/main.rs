use password_manager::*;
use passwords::PasswordGenerator;

#[tokio::main]
async fn main() {
    let pg = PasswordGenerator {
        length: 8,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    let password = Password::new("password".to_string(), "password".to_string());
    let exposed = password.exposed().await.unwrap();
    println!("Password: {}", password.password());
    println!("Exposed: {}", exposed);
    println!("Is common? {}", password.is_common());
    let mut pm = match PasswordManager::new(password.password()) {
        Ok(pm) => pm,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    let text = Text::new("label".to_string(), "test".to_string(), true);

    let mut record = Record::new("Testing".to_string(), Category::Login);
    record.set_content(RecordContent {
        main: MainItem::Login(
            Login::new(
                "example.com".to_string(),
                "username".to_string(),
                pg.generate_one().unwrap().to_string(),
            )
            .unwrap(),
        ),
        additional: Vec::new(),
    });

    record
        .edit_content()
        .unwrap()
        .additional
        .push(AdditionalItem::Text(text));

    pm.save_record(record).unwrap();

    pm.get_all_records().unwrap();

    let mut record = match pm.get_record(1) {
        Ok(record) => {
            println!("Record: {:?}", record);
            record
        }
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    pm.load_record_content(&mut record).unwrap();
    println!("Record: {:?}", record);
    record.delete_content();
    println!("Record: {:?}", record);
}
