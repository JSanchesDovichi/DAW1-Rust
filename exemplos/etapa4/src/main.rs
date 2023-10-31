use rocket::launch;

pub mod classes;
pub mod enums;
pub mod conexao;

#[launch]
async fn rocket() -> _ {
    let Ok(database_handler) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    rocket::build()
    .manage(database_handler)
}