use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Emprestimo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub data_hora_emprestimo: DateTime,
    pub data_hora_devolucao: Option<DateTime>,

    pub chave: DocumentoLigado,
    pub servidor_retirada: DocumentoLigado,
    pub servidor_devolucao: Option<DocumentoLigado>,

    pub ativo: bool
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DocumentoLigado {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub nome: String
}