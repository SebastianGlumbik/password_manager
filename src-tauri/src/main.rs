fn main() {
    if let Err(error) = password_manager::run() {
        eprintln!("{}", error);
    }
}
