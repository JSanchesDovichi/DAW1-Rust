use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Servidor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    nome: String,
    cpf: String,
    contato: String,
    nascimento: DateTime,
    ativo: bool,
}