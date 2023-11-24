use crate::classes::chave::Chave;
use crate::enums::EstadoChave;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateOptions;
use mongodb::{Collection, Database};
use rocket::futures::StreamExt;
use rocket::log::private::error;
use rocket::State;

pub struct ColecaoChaves {
    pub colecao: Collection<Chave>,
}

impl ColecaoChaves {
    pub fn default(database: &State<Database>) -> Self {
        ColecaoChaves {
            colecao: database.collection("Chaves"),
        }
    }

    pub async fn listar_chaves(&self) -> Vec<Chave> {
        let filtro = doc![
            "ativo": true
        ];

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
            }
            Err(e) => {
                println!("Erro ao criar/atualizar chave: {e}");
            }
        }

        None
    }

    pub async fn alterar_chave(&self, nome_original_chave: &String, nome_novo_chave: &String, estado_novo_chave: String) -> bool {
        let filtro = doc![
            "nome": &nome_original_chave
        ];

        let atualizacao = doc![
            "$set": doc![
                "nome": nome_novo_chave,
                "situacao": estado_novo_chave,
            ]
        ];

        match self.colecao.update_one(filtro, atualizacao, None).await {
            Ok(resultado) => {
                if resultado.modified_count == 1 {
                    return true;
                }
            }
            Err(e) => {
                println!("Erro ao criar/atualizar chave: {e}");
            }
        }

        false
    }

    pub async fn remover_chave(&self, nome_chave: String) -> bool {
        let filtro = doc![
            "nome": nome_chave
        ];

        let atualizacao = doc![
            "$set": doc![
                "ativo": false,
            ]
        ];

        match self.colecao.update_one(filtro, atualizacao, None).await {
            Ok(resultado) => {
                if resultado.modified_count == 1 {
                    return true;
                }
            }
            Err(e) => {
                println!("Erro ao remover chave: {e}");
            }
        }

        false
    }
}
