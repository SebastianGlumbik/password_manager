#[tokio::main]
async fn main() {
    if let Err(error) = password_manager::run().await {
        eprintln!("{}", error);
    }
}
