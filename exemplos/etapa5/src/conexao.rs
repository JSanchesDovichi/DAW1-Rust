use mongodb::Database;
use mongodb::options::Credential;
use mongodb::{Client, options::ClientOptions};
use mongodb::error::Result;

pub async fn get_database() -> Result<Database> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    client_options.credential = Some(Credential::builder()
    .username("root".to_string())
    .password("example".to_string()).build());

    match Client::with_options(client_options) {
        Ok(cliente) => {
            let db = cliente.database("daw1-rust");

            return Ok(db);
        },
        Err(e) => {
            return Err(e);
        }
    }
}