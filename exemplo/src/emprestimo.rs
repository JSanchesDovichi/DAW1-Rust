use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Emprestimo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    data_hora_emprestimo: DateTime,
    data_hora_devolucao: DateTime,
    ativo: bool
}