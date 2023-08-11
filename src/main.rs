use password_manager::{Login, PasswordManager};
use passwords::PasswordGenerator;
use secrecy::{ExposeSecret, SecretString};

fn main() {
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
    let password = SecretString::new("password".to_string());
    let pm = match PasswordManager::new(password.expose_secret()) {
        Ok(pm) => pm,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let mut login = match Login::new(
        String::from("https://www.example.com"),
        String::from("example@email.com"),
        pg.generate_one().unwrap(),
    ) {
        Ok(login) => login,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    if let Err(e) = login.set_totp_secret("pbopfriytzwfp3jfdiye2s4qoqb5rau5".to_string()) {
        println!("Error: {e}");
        return;
    }
    if let Err(e) = pm.save_login(login) {
        println!("Error: {e}");
        return;
    }
    let mut login = match pm.get_login(1) {
        Ok(login_info) => {
            println!("Login Info: {login_info:?}");
            login_info
        }
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let (otp, time) = match login.get_code() {
        Ok((otp, time)) => (otp, time),
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    println!("{otp} {time}");
    login.set_note("This is a note".to_string());
    if let Err(e) = pm.save_login(login) {
        println!("Error: {e}");
        return;
    }
    match pm.get_login(1) {
        Ok(login) => {
            println!("Login Info: {login:?}");
        }
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    let items = match pm.get_items() {
        Ok(items) => items,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    for item in items {
        println!("{item:?}");
    }
}
