use rocket::launch;
use rocket_cors::CorsOptions;
use rocket_dyn_templates::Template;

mod classes;
mod conexao;
mod enums;
mod rotas;
mod dao;

use rotas::chaves;

#[launch]
async fn rocket() -> _ {
    let Ok(database_handler) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    let Ok(cors_options) = CorsOptions::default().to_cors() else {
        std::process::exit(0);
    };

    rocket::build()
    .attach(Template::fairing())
        .attach(cors_options)
        .manage(database_handler)
        .mount("/chaves", chaves::rotas())
}
