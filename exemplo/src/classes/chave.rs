use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum EstadoChave {
    Emprestada,
    Disponivel
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Chave {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub nome: String,

    #[serde(skip_deserializing)]
    pub situacao: EstadoChave,

    pub ativo: bool
}

impl Default for EstadoChave {
    fn default() -> Self {
        EstadoChave::Disponivel
    }
}