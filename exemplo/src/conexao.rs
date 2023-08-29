use mongodb::Database;
use mongodb::options::Credential;

pub async fn get_database() -> Option<Database> {
    use mongodb::{Client, options::ClientOptions};

    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    client_options.credential = Some(Credential::builder()
        .username("root".to_string())
        .password("example".to_string()).build());


    if let Ok(cliente) = Client::with_options(client_options) {
        let db = cliente.database("daw1-rust");

        Some(db)
    } else {
        None
    }
}