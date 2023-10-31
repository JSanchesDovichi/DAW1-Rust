use crate::classes::chave::Chave;
use mongodb::bson::oid::ObjectId;
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

}