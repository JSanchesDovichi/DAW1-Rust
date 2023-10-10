# Implementação de classes:

# Criação da pasta de classes do projeto:
Crie a pasta src/classes e o arquivo mod.rs dentro dela: adicione as importações para os arquivos:
```
pub mod emprestimo;
pub mod chave;
pub mod servidor;
```

# Chave
(Crie o arquivo src/classes/chave.rs)
```
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
```

# Servidor
(Crie o arquivo src/classes/servidor.rs)
```
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Servidor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub nome: String,
    pub cpf: String,
    pub contato: String,
    pub nascimento: DateTime,
    pub ativo: bool,
}
```

# Emprestimo
(Crie o arquivo src/classes/emprestimo.rs)
```
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
```

### Importar as classes no projeto
Adicone no arquivo src/main.rs:
```
mod classes;
```