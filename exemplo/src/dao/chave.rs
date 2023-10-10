use crate::classes::chave::Chave;
use crate::enums::EstadoChave;
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateOptions;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use rocket::log::private::error;
use rocket::serde::json::Json;
use rocket::State;
use rocket::futures::StreamExt;
use rocket::http::Status;
use std::str::FromStr;

pub struct ColecaoChaves {
    pub colecao: Collection<Chave>,
}

impl ColecaoChaves {
    pub fn default(database: &State<Database>) -> Self {
        ColecaoChaves {
            colecao: database.collection("Chaves")
        }
    }

    pub async fn listar_chaves(&self) -> Vec<Chave> {
        let filtro = doc![];

        match self.colecao.find(filtro, None).await {
            Ok(resultados) => {
                let mut lista_chaves_encontradas = vec![];

                let map = resultados.map(|pessoa| pessoa);
                let chaves_encontradas = map.collect::<Vec<_>>().await;

                for chave_encontrada in chaves_encontradas.into_iter().flatten() {
                    lista_chaves_encontradas.push(chave_encontrada);
                }

                lista_chaves_encontradas
            }
            Err(e) => {
                error!("{e}");
                vec![]
            }
        }
    }

    pub async fn buscar_chave(&self, id: &str) -> Option<Chave> {
        let Ok(id_convertido) = ObjectId::from_str(id) else {
            return None;
        };

        let filtro = doc![
            "_id": id_convertido
        ];

        match self.colecao.find_one(filtro, None).await {
            Ok(chave_encontrada) => {
                chave_encontrada
            }
            Err(e) => {
                println!("{e}");
                None
            }
        }
    }

    pub async fn adicionar_chave(&self, nome_chave_nova: &str) -> Option<ObjectId> {
        let filtro = doc![
            "nome": &nome_chave_nova
        ];

        let atualizacao = doc![
            "$set": doc![
                "situacao": EstadoChave::Disponivel,
                "ativo": true
            ]
        ];

        let opcoes = UpdateOptions::builder().upsert(true).build();

        match self.colecao.update_one(filtro, atualizacao, opcoes).await {
            Ok(resultado) => {

                if let Some(id_atualizaco) = resultado.upserted_id {
                    return id_atualizaco.as_object_id();
                }
            },
            Err(e) => {
                println!("Erro ao criar/atualizar chave: {e}");

            }
        }

        None
    }

    pub async fn remover_chave(&self, chave: Json<Chave>) -> Status {
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

    pub async fn atualizar_chave(&self, chave: Json<Chave>) -> Status {
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