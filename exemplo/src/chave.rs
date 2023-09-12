use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
enum EstadoChave {
    Emprestada,
    Disponivel
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Chave {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    nome: String,
    situacao: EstadoChave,
    ativo: bool
}