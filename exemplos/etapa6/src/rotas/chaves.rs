use mongodb::bson::doc;
use mongodb::Database;
use rocket::{Route, routes, get, State, put, delete, patch};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::classes::chave::{Chave};
use crate::classes::chave::ChaveParaCriacao;
use crate::dao::chave::ColecaoChaves;
use rocket_dyn_templates::{context, Template};

pub fn rotas() -> Vec<Route> {
    routes![
        listar_chaves,
        pagina_criar_chave,
        criar_chave
    ]
}

#[get("/")]
async fn listar_chaves(database: &State<Database>) -> Template {
    let lista_chaves = ColecaoChaves::default(database).listar_chaves().await;

    Template::render("chaves", context! {
        chaves: lista_chaves
    })
}

#[get("/nova")]
async fn pagina_criar_chave() -> Template {
    Template::render("criar_chave", context! {})
 }

#[put("/", data = "<chave_para_criacao>")]
async fn criar_chave(database: &State<Database>, chave_para_criacao: Json<ChaveParaCriacao>) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves.adicionar_chave(&chave_para_criacao.nome).await.is_some() {
        Status::Ok
    } else {
        Status::Conflict
    }
 }