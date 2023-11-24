use std::fmt::Debug;
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

use crate::enums::EstadoChave; 

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Chave {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub nome: String,

    //#[serde(skip_deserializing)]
    pub situacao: EstadoChave,

    pub ativo: bool
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ChaveParaCriacao {
    pub nome: String,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ChaveParaEdicao {
   pub nome_original: String,
   pub nome_novo: String,
   pub estado_novo: EstadoChave
}