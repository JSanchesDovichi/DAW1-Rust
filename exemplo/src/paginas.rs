use mongodb::bson::DateTime;
use rocket::{get, Route, routes};
use rocket_dyn_templates::{context, Template};
use crate::classes::servidor::Servidor;

pub fn rotas() -> Vec<Route> {
    routes![
        index
    ]
}

#[get("/index")]
async fn index() -> Template {
    let mut lista_teste_pessoas: Vec<Servidor> = vec![];

    lista_teste_pessoas.push(
        Servidor {
            ativo: true,
            id: None,
            nome: "João Pedro Sanches Dovichi".to_string(),
            cpf: "Teste".to_string(),
            contato: "telefone".to_string(),
            nascimento: DateTime::now()
        }
        /*
        Pessoa{
            id: None,
            nome: "João Pedro Sanches".to_string(),
            idade: 25
        }
         */
    );

    lista_teste_pessoas.push(
        Servidor {
            ativo: true,
            id: None,
            nome: "Alex Ferreira".to_string(),
            cpf: "Teste".to_string(),
            contato: "telefone".to_string(),
            nascimento: DateTime::now()
        }
    );

    lista_teste_pessoas.push(
        Servidor {
            ativo: true,
            id: None,
            nome: "Paulo Eduardo".to_string(),
            cpf: "Teste".to_string(),
            contato: "telefone".to_string(),
            nascimento: DateTime::now()
        }
    );

    Template::render("index", context! {
        pessoas: lista_teste_pessoas
    })
}

