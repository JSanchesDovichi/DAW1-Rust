# Implemenção do DAO de Chaves

## Crie os arquivos

* Para criar as classes DAO, primeiro crie a pasta dao dentro da pasta src:

```diff
.
├── Cargo.toml
├── Rocket.toml
└── src
    ├── classes
    │   ├── chave.rs
    │   ├── emprestimo.rs
    │   ├── mod.rs
    │   └── servidor.rs
    ├── conexao.rs
+  ├── dao
    ├── enums.rs
    └── main.rs

```

* Agora dentro da pasta dao, crie os arquivos mod.rs e chave.rs:

```diff
.
├── Cargo.toml
├── Rocket.toml
└── src
    ├── classes
    │   ├── chave.rs
    │   ├── emprestimo.rs
    │   ├── mod.rs
    │   └── servidor.rs
    ├── conexao.rs
    ├── dao
+  │   ├── chave.rs
+  │   └── mod.rs
    ├── enums.rs
    └── main.rs
```

* Adicione o cõdigo a seguir para o arquivo dao/mod.rs

```rust
pub mod chave;
```

> Arquivo exemplo disponível em [dao.rs](../exemplos/etapa5/src/dao/mod.rs)

* Adicione o código a seguir para o arquivo dao/chave.rs

```rust
use crate::classes::chave::Chave;
use mongodb::bson::oid::ObjectId;
use mongodb::{Collection, Database};
use mongodb::bson::doc;
use rocket::log::private::error;
use rocket::serde::json::Json;
use rocket::State;
use rocket::futures::StreamExt;
use rocket::http::Status;
use std::str::FromStr;

pub struct ColecaoChaves {
    pub colecao: Collection<Chave>,
}

impl ColecaoChaves {
    pub fn default(database: &State<Database>) -> Self {
        ColecaoChaves {
            colecao: database.collection("Chaves")
        }
    }

    pub async fn listar_chaves(&self) -> Vec<Chave> {
        let filtro = doc![];

        match self.colecao.find(filtro, None).await {
            Ok(resultados) => {
                let mut lista_chaves_encontradas = vec![];

                let map = resultados.map(|pessoa| pessoa);
                let chaves_encontradas = map.collect::<Vec<_>>().await;

                for chave_encontrada in chaves_encontradas.into_iter().flatten() {
                    lista_chaves_encontradas.push(chave_encontrada);
                }

                lista_chaves_encontradas
            }
            Err(e) => {
                error!("{e}");
                vec![]
            }
        }
    }

}
```

> Arquivo exemplo disponível em [chave.rs](../exemplos/etapa5/src/dao/chave.rs)

* Crie uma pasta chamada paginas dentro da pasta src:

```diff
.
├── Cargo.toml
├── Rocket.toml
└── src
    ├── classes
    │   ├── chave.rs
    │   ├── emprestimo.rs
    │   ├── mod.rs
    │   └── servidor.rs
    ├── conexao.rs
    ├── dao
    │   ├── chave.rs
    │   └── mod.rs
    ├── enums.rs
    ├── main.rs
+  └── paginas
```

* Dentro da pasta paginas crie o arquivo chaves.html.tera:
  
```diff
.
├── Cargo.toml
├── Rocket.toml
└── src
    ├── classes
    │   ├── chave.rs
    │   ├── emprestimo.rs
    │   ├── mod.rs
    │   └── servidor.rs
    ├── conexao.rs
    ├── dao
    │   ├── chave.rs
    │   └── mod.rs
    ├── enums.rs
    ├── main.rs
    └── paginas
+         └── chaves.html.tera
```

* Adicione o seguinte código ao arquivo paginas/chaves.html.tera:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>Chaves disponíveis:</title>
  <style>
    body {
        font-family: Arial, sans-serif;
        background-color: #f7f7f7;
    }

    h1 {
        color: #333;
        text-align: center;
        margin-top: 50px;
        margin-bottom: 30px;
    }

    table {
        margin: 0 auto;
        border: 2px solid #333;
        border-collapse: collapse;
        background-color: #fff;
        width: 80%;
        max-width: 800px;
    }

    th {
        background-color: #333;
        color: #fff;
        padding: 10px;
        font-size: 18px;
        text-align: left;
    }

    td {
        border: 2px solid #333;
        padding: 10px;
        font-size: 16px;
    }

    tr:nth-child(even) {
        background-color: #f2f2f2;
    }

    tr:hover {
        background-color: #ddd;
    }
  </style>
</head>
<body>
<section id="printable">
  <h1>Chaves</h1>
  <table>
    <thead>
    <tr>
      <th>Nome</th>
      <th>Situação</th>
    </tr>
    </thead>
    <tbody>
    {% for chave in chaves %}
    <tr>
      <td>{{ chave.nome }}</td>
      <td>{{ chave.situacao }}</td>
    </tr>
    {% endfor %}
    </tbody>
  </table>
</section>

</body>
</html>
```

> Arquivo exemplo disponível em [chaves.html.tera](../exemplos/etapa5/src/paginas/chaves.html.tera)

* Crie a pasta rotas dentro da pasta src do projeto:

```diff
.
├── Cargo.toml
├── Rocket.toml
└── src
    ├── classes
    │   ├── chave.rs
    │   ├── emprestimo.rs
    │   ├── mod.rs
    │   └── servidor.rs
    ├── conexao.rs
    ├── dao
    │   ├── chave.rs
    │   └── mod.rs
    ├── enums.rs
    ├── main.rs
    ├── paginas
    │   └── chaves.html.tera
+  └── rotas
```

* Dentro da pasta rotas crie os arquivos mod.rs e chaves.rs:

```diff
.
├── Cargo.toml
├── Rocket.toml
└── src
    ├── classes
    │   ├── chave.rs
    │   ├── emprestimo.rs
    │   ├── mod.rs
    │   └── servidor.rs
    ├── conexao.rs
    ├── dao
    │   ├── chave.rs
    │   └── mod.rs
    ├── enums.rs
    ├── main.rs
    ├── paginas
    │   └── chaves.html.tera
    └── rotas
+          ├── chaves.rs
+          └── mod.rs
```

* No arquivo rotas/mod.rs adicione o seguinte código:

```rust
pub mod chaves;
```

> Arquivo exemplo disponível em [mod.rs](../exemplos/etapa5/src/rotas/mod.rs)

* No arquivo rotas/chaves.rs adicione o seguinte código:

```rust
use mongodb::bson::doc;
use mongodb::Database;
use rocket::{Route, routes, get, State, put, delete, patch};
use rocket::http::Status;
use rocket::serde::json::Json;
use crate::classes::chave::{Chave};
use crate::dao::chave::ColecaoChaves;
use rocket_dyn_templates::{context, Template};

pub fn rotas() -> Vec<Route> {
    routes![
        listar_chaves,
    ]
}

#[get("/")]
async fn listar_chaves(database: &State<Database>) -> Template {
    let lista_chaves = ColecaoChaves::default(database).listar_chaves().await;

    Template::render("chaves", context! {
        chaves: lista_chaves
    })
}
```

> Arquivo exemplo disponível em [chaves.rs](../exemplos/etapa5/src/dao/chaves.rs)

* Agora importe todo o código novo para o arquivo main.rs:

```diff
use rocket::launch;
use rocket_cors::CorsOptions;
+ use rocket_dyn_templates::Template;

mod classes;
mod conexao;
mod enums;
+ mod rotas;
+ mod dao;

+ use rotas::chaves;

#[launch]
async fn rocket() -> _ {
    let Ok(database_handler) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    let Ok(cors_options) = CorsOptions::default().to_cors() else {
        std::process::exit(0);
    };

    rocket::build()
+        .attach(Template::fairing())
        .attach(cors_options)
        .manage(database_handler)
+        .mount("/chaves", chaves::rotas())
}
```

> Arquivo exemplo disponível em [main.rs](../exemplos/etapa5/src/main.rs)

* Para testar esta etapa acesse o site <http://127.0.0.1:8080/chaves> para ver a listagem de chaves. Nenhuma chave foi cadastrada até o momento, portanto, a listagem deve aparecer vazia.
  