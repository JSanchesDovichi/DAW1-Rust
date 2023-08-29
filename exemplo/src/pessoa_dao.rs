use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use rocket::log::private::error;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use rocket::futures::StreamExt;
use rocket::http::Status;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pessoa {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    nome: String,
    idade: i32,
}

pub struct ColecaoPessoas {
    pub colecao: Collection<Pessoa>,
}

impl ColecaoPessoas {
    pub fn default(database: &State<Database>) -> Self {
        ColecaoPessoas {
            colecao: database.collection("Pessoas")
        }
    }

    pub async fn buscar_pessoas(&self) -> Vec<Pessoa> {
        let filtro = doc![];

        match self.colecao.find(filtro, None).await {
            Ok(resultados) => {
                let mut lista_pessoas_encontradas = vec![];

                let map = resultados.map(|pessoa| pessoa);
                let pessoas_encontradas = map.collect::<Vec<_>>().await;

                for pessoa_recuperada in pessoas_encontradas.into_iter().flatten() {
                    lista_pessoas_encontradas.push(pessoa_recuperada);
                }

                lista_pessoas_encontradas
            }
            Err(e) => {
                error!("{e}");
                vec![]
            }
        }
    }

    pub async fn buscar_pessoa(&self, id: &str) -> Option<Pessoa> {
        let Ok(id_convertido) = ObjectId::from_str(id) else {
            return None;
        };

        let filtro = doc![
            "_id": id_convertido
        ];

        match self.colecao.find_one(filtro, None).await {
            Ok(pessoa_encontrada) => {
                pessoa_encontrada
            }
            Err(e) => {
                println!("{e}");
                None
            }
        }
    }

    pub async fn adicionar_pessoa(&self, pessoa_nova: Json<Pessoa>) -> Option<ObjectId> {
        match self.colecao.insert_one(&*pessoa_nova, None).await {
            Ok(resultado) => {
                resultado.inserted_id.as_object_id()
            }
            Err(e) => {
                println!("{e}");
                None
            }
        }
    }

    pub async fn remover_pessoa(&self, pessoa: Json<Pessoa>) -> Status {
        let filtro = doc![
            "nome": &pessoa.nome
        ];

        match self.colecao.delete_one(filtro, None).await {
            Ok(resultado) => {
                if resultado.deleted_count == 1 {
                    Status::Ok
                } else {
                    Status::NotFound
                }
            }
            Err(e) => {
                println!("{e}");
                Status::InternalServerError
            }
        }
    }

    pub async fn atualizar_pessoa(&self, pessoa: Json<Pessoa>) -> Status {
        let filtro = doc![
            "_id": pessoa.id
        ];

        let atualizacao = doc![
            "$set": doc![
                 "nome": &pessoa.nome,
            "idade": pessoa.idade
            ]
        ];

        match self.colecao.update_one(filtro, atualizacao, None).await {
            Ok(resultado) => {
                if resultado.modified_count == 1 {
                    Status::Ok
                } else {
                    Status::NotFound
                }
            }
            Err(e) => {
                println!("{e}");
                Status::InternalServerError
            }
        }
    }
}