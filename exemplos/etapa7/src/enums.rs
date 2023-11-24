use std::fmt;
use std::fmt::{Debug, Display};
use mongodb::bson::Bson;
use rocket::serde::{Deserialize, Serialize}; 

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(crate = "rocket::serde")]
pub enum EstadoChave {
    #[default]
    Disponivel,

    Emprestada
}

impl Display for EstadoChave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<EstadoChave> for Bson {
    fn from(item: EstadoChave) -> Self {
        Bson::String(item.to_string())
    }
}