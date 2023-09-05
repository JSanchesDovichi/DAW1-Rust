use rocket::{launch};
use rocket_dyn_templates::Template;

mod conexao;
mod pessoas;
mod pessoa_dao;
mod paginas;
mod chave;

mod servidor;

mod emprestimo;

#[launch]
async fn rocket() -> _ {
    let Some(handler_database) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    rocket::build()
        .attach(Template::fairing())
        .manage(handler_database)
        .mount("/", paginas::rotas())
        .mount("/pessoas", pessoas::rotas())
}