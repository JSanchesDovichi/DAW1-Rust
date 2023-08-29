use rocket::{launch};

mod conexao;
mod pessoas;
mod pessoa_dao;

#[launch]
async fn rocket() -> _ {
    let Some(handler_database) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    rocket::build()
        .manage(handler_database)
        .mount("/pessoas", pessoas::rotas())
}