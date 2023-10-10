use mongodb::bson::doc;
use mongodb::Database;
use rocket::{Route, routes, get, State, put, delete, patch};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::classes::chave::{Chave, ChaveParaCriacao};
use crate::dao::chave::ColecaoChaves;
use rocket_dyn_templates::{context, Template};

pub fn rotas() -> Vec<Route> {
    routes![
        listar_chaves,
        buscar_chave,
        pagina_criar_chave,
        remover_chave,
        atualizar_chave,
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

#[get("/<id>")]
async fn buscar_chave(database: &State<Database>, id: &str) -> (Status, Json<Option<Chave>>) {
    let colecao_chaves = ColecaoChaves::default(database);

    //return colecao_pessoas.buscar_pessoa(pessoa).await;
    if let Some(chave) = colecao_chaves.buscar_chave(id).await {
        return (Status::Found, Json(Some(chave)));
    }

    (Status::NotFound, Json(None))
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

#[delete("/", data = "<chave_remocao>")]
async fn remover_chave(database: &State<Database>, chave_remocao: Json<Chave>) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    colecao_chaves.remover_chave(chave_remocao).await
}

#[patch("/", data = "<chave_atualizacao>")]
async fn atualizar_chave(database: &State<Database>, chave_atualizacao: Json<Chave>) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    colecao_chaves.atualizar_chave(chave_atualizacao).await
}