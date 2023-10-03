use rocket::{launch};
use rocket_dyn_templates::Template;

mod conexao;
mod paginas;
mod classes;

mod rotas;

mod dao;

use rotas::{chaves, servidor};

#[launch]
async fn rocket() -> _ {
    let Some(handler_database) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    rocket::build()
        .attach(Template::fairing())
        .manage(handler_database)
        .mount("/", paginas::rotas())
        .mount("/chaves", chaves::rotas())
        .mount("/servidores", servidor::rotas())
}