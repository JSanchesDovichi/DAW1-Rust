# Implementação de classes:

# Chave
```
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
```

# Servidor
```
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Servidor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    nome: String,
    cpf: String,
    contato: String,
    nascimento: DateTime,
    ativo: bool,
}
```

# Emprestimo
```
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Emprestimo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,

    data_hora_emprestimo: DateTime,
    data_hora_devolucao: DateTime,
    ativo: bool
}
```