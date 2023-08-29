use rocket::{get, Route, routes};
use rocket_dyn_templates::{context, Template};
use crate::pessoa_dao::Pessoa;

pub fn rotas() -> Vec<Route> {
    routes![
        index
    ]
}

#[get("/index")]
async fn index() -> Template {
    let mut lista_teste_pessoas: Vec<Pessoa> = vec![];

    lista_teste_pessoas.push(
        Pessoa{
            id: None,
            nome: "João Pedro Sanches".to_string(),
            idade: 25
        }
    );

    lista_teste_pessoas.push(
        Pessoa{
            id: None,
            nome: "Alex Ferreira".to_string(),
            idade: 25
        }
    );

    lista_teste_pessoas.push(
        Pessoa{
            id: None,
            nome: "Paulo Eduardo".to_string(),
            idade: 25
        }
    );

    Template::render("index", context! {
        pessoas: lista_teste_pessoas
    })
}

