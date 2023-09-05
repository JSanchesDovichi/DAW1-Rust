use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
enum EstadoChave {
    Emprestada,
    Disponivel
}

#[derive(Deserialize, Serialize, Debug)]
struct Chave {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    nome: String,
    situacao: EstadoChave,
    ativo: bool
}