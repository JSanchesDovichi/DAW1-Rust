# Implemenção da classes

## Crie os arquivos

* Para criar as classes, primeiro crie a pasta classes dentro da pasta src:

```diff
    .
    ├── Cargo.toml
    ├── Rocket.toml
    └── src
+          ├── classes
            └── main.rs

```

* Agora crie dentro dessa pasta os arquivos mod.rs, chave.rs, emprestimo.rs e servidor.rs,
* Crie também o arquivo enums.rs na mesma pasta do arquivo main.rs

```diff
.
├── Cargo.toml
├── Rocket.toml
└── src
+      ├── classes
+      │   ├── chave.rs
+      │   ├── emprestimo.rs
+      │   ├── mod.rs
+      │   └── servidor.rs
+      ├── enums.rs
        └── main.rs

```

* Adicione o cõdigo a seguir para o arquivo mod.rs

```rust
pub mod emprestimo;
pub mod chave;
pub mod servidor;
```

> Arquivo exemplo disponível em [mod.rs](../exemplos/etapa3/src/classes/mod.rs)

* Adicione o código a seguir para o arquivo chave.rs

```rust
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

    #[serde(skip_deserializing)]
    pub situacao: EstadoChave,

    pub ativo: bool
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ChaveParaCriacao {
    pub nome: String,
}
```

> Arquivo exemplo disponível em [chave.rs](../exemplos/etapa3/src/classes/chave.rs)

* Adicione o código a seguir para o arquivo servidor.rs

```rust
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

> Arquivo exemplo disponível em [servidor.rs](../exemplos/etapa3/src/classes/servidor.rs)

* Adicione o código a seguir para o arquivo emprestimo.rs

```rust
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

> Arquivo exemplo disponível em [emprestimo.rs](../exemplos/etapa3/src/classes/emprestimo.rs)

* Adicione o código a seguir para o arquivo enums.rs

```rust
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
```

> Arquivo exemplo disponível em [enums.rs](../exemplos/etapa3/src/classes/enums.rs)

* Por fim, adicione a importação para essas classes e a enumeração no arquivo main.rs:
  
```diff
use rocket::launch;

+ mod classes;
+ mod enums;

#[launch]
async fn rocket() -> _ {
    rocket::build()
}
```

> Arquivo exemplo disponível em [main.rs](../exemplos/etapa3/src/main.rs)
