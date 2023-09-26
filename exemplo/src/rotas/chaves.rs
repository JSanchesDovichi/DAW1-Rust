use mongodb::Database;
use rocket::{Route, routes, get, State, put, delete, patch};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::classes::chave::Chave;
use crate::dao::chave::ColecaoChaves;

pub fn rotas() -> Vec<Route> {
    routes![
        listar_chaves,
        buscar_chave,
        criar_chave,
        remover_chave,
        atualizar_chave
    ]
}

#[get("/")]
async fn listar_chaves(database: &State<Database>) {
    let colecao_chaves = ColecaoChaves::default(database);

    let lista_chaves = colecao_chaves.listar_chaves().await;

    for pessoa in lista_chaves {
        println!("{:?}", pessoa);
    }
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

#[put("/", data = "<chave_nova>")]
async fn criar_chave(database: &State<Database>, chave_nova: Json<Chave>) -> Option<String> {
    let colecao_chaves = ColecaoChaves::default(database);

    if let Some(id_inserido) = colecao_chaves.adicionar_chave(chave_nova).await {
        return Some(id_inserido.to_string());
    }

    None
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