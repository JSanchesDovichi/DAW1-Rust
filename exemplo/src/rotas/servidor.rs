use mongodb::Database;
use rocket::{Route, routes, get, State, put, delete, patch};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::classes::chave::Chave;
use crate::classes::servidor::Servidor;
use crate::dao::servidor::ColecaoServidores;

pub fn rotas() -> Vec<Route> {
    routes![
        listar_servidores,
        buscar_servidor,
        criar_servidor,
        remover_servidor,
        atualizar_servidor
    ]
}

#[get("/")]
async fn listar_servidores(database: &State<Database>) {
    let colecao_servidores = ColecaoServidores::default(database);

    let lista_servidores = colecao_servidores.listar_servidores().await;

    for servidor in lista_servidores {
        println!("{:?}", servidor);
    }
}

#[get("/<id>")]
async fn buscar_servidor(database: &State<Database>, id: &str) -> (Status, Json<Option<Servidor>>) {
    let colecao_servidores = ColecaoServidores::default(database);

    if let Some(servidor) = colecao_servidores.buscar_servidor(id).await {
        return (Status::Found, Json(Some(servidor)));
    }

    (Status::NotFound, Json(None))
}

#[put("/", data = "<servidor_novo>")]
async fn criar_servidor(database: &State<Database>, servidor_novo: Json<Servidor>) -> Option<String> {
    let colecao_servidores = ColecaoServidores::default(database);

    if let Some(id_inserido) = colecao_servidores.adicionar_servidor(servidor_novo).await {
        return Some(id_inserido.to_string());
    }

    None
}

#[delete("/", data = "<servidor_remocao>")]
async fn remover_servidor(database: &State<Database>, servidor_remocao: Json<Servidor>) -> Status {
    let colecao_servidores = ColecaoServidores::default(database);

    colecao_servidores.remover_servidor(servidor_remocao).await
}

#[patch("/", data = "<servidor_atualizacao>")]
async fn atualizar_servidor(database: &State<Database>, servidor_atualizacao: Json<Servidor>) -> Status {
    let colecao_servidores = ColecaoServidores::default(database);

    colecao_servidores.atualizar_servidor(servidor_atualizacao).await
}