use mongodb::Database;
use rocket::{Route, routes, get, State, put, delete, patch};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::pessoa_dao::{ColecaoPessoas, Pessoa};

pub fn rotas() -> Vec<Route> {
    routes![
        listar_pessoas,
        buscar_pessoa,
        criar_pessoa,
        remover_pessoa,
        atualizar_pessoa
    ]
}

#[get("/")]
async fn listar_pessoas(database: &State<Database>) {
    let colecao_pessoas = ColecaoPessoas::default(database);

    let lista_pessoas = colecao_pessoas.buscar_pessoas().await;

    for pessoa in lista_pessoas {
        println!("{:?}", pessoa);
    }
}

#[get("/<id>")]
async fn buscar_pessoa(database: &State<Database>, id: &str) -> (Status, Json<Option<Pessoa>>) {
    let colecao_pessoas = ColecaoPessoas::default(database);

    //return colecao_pessoas.buscar_pessoa(pessoa).await;
    if let Some(pessoa) = colecao_pessoas.buscar_pessoa(id).await {
        return (Status::Found, Json(Some(pessoa)));
    }

    (Status::NotFound, Json(None))
}

#[put("/", data = "<pessoa_nova>")]
async fn criar_pessoa(database: &State<Database>, pessoa_nova: Json<Pessoa>) -> Option<String> {
    let colecao_pessoas = ColecaoPessoas::default(database);

    if let Some(id_inserido) = colecao_pessoas.adicionar_pessoa(pessoa_nova).await {
        return Some(id_inserido.to_string());
    }

    None
}

#[delete("/", data = "<pessoa_remocao>")]
async fn remover_pessoa(database: &State<Database>, pessoa_remocao: Json<Pessoa>) -> Status {
    let colecao_pessoas = ColecaoPessoas::default(database);

    colecao_pessoas.remover_pessoa(pessoa_remocao).await
}

#[patch("/", data = "<pessoa_atualizacao>")]
async fn atualizar_pessoa(database: &State<Database>, pessoa_atualizacao: Json<Pessoa>) -> Status {
    let colecao_pessoas = ColecaoPessoas::default(database);

    colecao_pessoas.atualizar_pessoa(pessoa_atualizacao).await
}