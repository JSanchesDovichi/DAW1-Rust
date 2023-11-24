use crate::classes::chave::ChaveParaCriacao;
use crate::classes::chave::ChaveParaEdicao;
use crate::dao::chave::ColecaoChaves;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, put, routes, Route, State};
use rocket_dyn_templates::{context, Template};

pub fn rotas() -> Vec<Route> {
    routes![listar_chaves, pagina_criar_chave, criar_chave, pagina_editar_chave, editar_chave, remover_chave]
}

#[get("/")]
async fn listar_chaves(database: &State<Database>) -> Template {
    let lista_chaves = ColecaoChaves::default(database).listar_chaves().await;

    Template::render(
        "chaves",
        context! {
            chaves: lista_chaves
        },
    )
}

#[get("/editar?<name>&<situacao>")]
async fn pagina_editar_chave(name: &str, situacao: &str) -> Template {
    Template::render(
        "editar_chave",
        context! {
            chave: doc![
                "nome": name,
                "estado": situacao
            ]
        },
    )
}

#[get("/nova")]
async fn pagina_criar_chave() -> Template {
    Template::render("criar_chave", context! {})
}

#[patch("/", data = "<chave_para_edicao>")]
async fn editar_chave(
    database: &State<Database>,
    chave_para_edicao: Json<ChaveParaEdicao>,
) -> Status {

    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves
        .alterar_chave(&chave_para_edicao.nome_original, &chave_para_edicao.nome_novo, chave_para_edicao.estado_novo.to_string())
        .await
        == true
    {
        Status::Ok
    } else {
        Status::Conflict
    }
}

#[put("/", data = "<chave_para_criacao>")]
async fn criar_chave(
    database: &State<Database>,
    chave_para_criacao: Json<ChaveParaCriacao>,
) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves
        .adicionar_chave(&chave_para_criacao.nome)
        .await
        .is_some()
    {
        Status::Ok
    } else {
        Status::Conflict
    }
}

#[delete("/<nome_chave>")]
async fn remover_chave(
    database: &State<Database>,
    nome_chave: String
) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves
        .remover_chave(nome_chave)
        .await
        == true
    {
        Status::Ok
    } else {
        Status::Conflict
    }
}
