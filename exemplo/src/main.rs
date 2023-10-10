use rocket::launch;
use rocket_cors::CorsOptions;
use rocket_dyn_templates::Template;

mod conexao;
mod paginas;
mod classes;
mod rotas;
mod dao;
mod enums;

use rotas::{chaves, servidor};

#[launch]
async fn rocket() -> _ {
    let Some(handler_database) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    let Ok(cors_options) = CorsOptions::default().to_cors() else {
        std::process::exit(0);
      };

    rocket::build()
        .attach(cors_options)
        .attach(Template::fairing())
        .manage(handler_database)
        .mount("/", paginas::rotas())
        .mount("/chaves", chaves::rotas())
        .mount("/servidores", servidor::rotas())
}