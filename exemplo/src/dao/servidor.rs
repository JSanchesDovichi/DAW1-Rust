use mongodb::bson::Bson::DateTime;
use mongodb::{bson};
use crate::classes::chave::Chave;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use rocket::log::private::error;
use rocket::serde::json::Json;
use rocket::State;
use rocket::serde::{Deserialize, Serialize};
use rocket::futures::StreamExt;
use rocket::http::Status;
use std::str::FromStr;
use crate::classes::servidor::Servidor;

pub struct ColecaoServidores {
    pub colecao: Collection<Servidor>,
}

impl ColecaoServidores {
    pub fn default(database: &State<Database>) -> Self {
        ColecaoServidores {
            colecao: database.collection("Pessoas")
        }
    }

    pub async fn listar_servidores(&self) -> Vec<Servidor> {
        let filtro = doc![];

        match self.colecao.find(filtro, None).await {
            Ok(resultados) => {
                let mut lista_servidores_encontrados = vec![];

                let map = resultados.map(|pessoa| pessoa);
                let servidores_encontrados = map.collect::<Vec<_>>().await;

                for servidor_encontrado in servidores_encontrados.into_iter().flatten() {
                    lista_servidores_encontrados.push(servidor_encontrado);
                }

                lista_servidores_encontrados
            }
            Err(e) => {
                error!("{e}");
                vec![]
            }
        }
    }

    pub async fn buscar_servidor(&self, id: &str) -> Option<Servidor> {
        let Ok(id_convertido) = ObjectId::from_str(id) else {
            return None;
        };

        let filtro = doc![
            "_id": id_convertido
        ];

        match self.colecao.find_one(filtro, None).await {
            Ok(servidor_encontrado) => {
                servidor_encontrado
            }
            Err(e) => {
                println!("{e}");
                None
            }
        }
    }

    pub async fn adicionar_servidor(&self, mut servidor_novo: Json<Servidor>) -> Option<ObjectId> {
        servidor_novo.ativo = true;

        match self.colecao.insert_one(&*servidor_novo, None).await {
            Ok(resultado) => {
                resultado.inserted_id.as_object_id()
            }
            Err(e) => {
                println!("{e}");
                None
            }
        }
    }

    pub async fn remover_servidor(&self, chave: Json<Servidor>) -> Status {
        let filtro = doc![
            "nome": &chave.nome
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

    pub async fn atualizar_servidor(&self, chave: Json<Servidor>) -> Status {
        let filtro = doc![
            "_id": chave.id
        ];

        let atualizacao = doc![
            "$set": doc![
                 "nome": &chave.nome,
                //"idade": chave.idade
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