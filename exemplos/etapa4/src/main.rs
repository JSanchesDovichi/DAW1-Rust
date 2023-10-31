use rocket::launch;
use rocket_cors::CorsOptions;

pub mod classes;
pub mod conexao;
pub mod enums;

#[launch]
async fn rocket() -> _ {
    let Ok(database_handler) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    let Ok(cors_options) = CorsOptions::default().to_cors() else {
        std::process::exit(0);
    };

    rocket::build()
        .attach(cors_options)
        .manage(database_handler)
}
