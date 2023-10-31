### Crie o arquivo de importação para os DAOs:
No arquivo src/dao/mod.rs
```
pub mod chave;
```

### Função CRUD para listar as chaves disponíveis no sistema:
(No arquivo src/dao/chave.rs)
```
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

### Template da página de chaves:
(Crie o arquivo src/paginas/chaves.html.tera)
```
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
<button id="button">Gerar pdf!</button>

<script src="https://cdnjs.cloudflare.com/ajax/libs/html2pdf.js/0.10.1/html2pdf.bundle.min.js" integrity="sha512-GsLlZN/3F2ErC5ifS5QtgpiJtWd43JWSuIgh7mbzZ8zBps+dvLusV+eNQATqgA/HdeKFVgA5v3S/cIrLF7QnIg==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>

<script>
  const btn = document.getElementById("button");

  btn.addEventListener("click", function(){
  var element = document.getElementById('printable');
  html2pdf().from(element).save('filename.pdf');
  });
</script>
</body>
</html>
```
### Crie o arquivo de importação para as rotas:
No arquivo src/rotas/mod.rs
```
pub mod chaves;
```


### Endpoint de acesso para o template:
(No arquivo src/rotas/chaves.rs)
```
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

### Crie o arquivo de enumerações
No arquivo enums.rs
```
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

### Adicionando a rota na api:
(No arquivo src/main.rs)
```
use rocket::{launch};
use rocket_dyn_templates::Template;
use rocket_cors::CorsOptions;

mod conexao;
mod classes;
mod rotas;
mod dao;
mod enums;

use rotas::chaves;

#[launch]
async fn rocket() -> _ {
    let Some(handler_database) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    let Ok(cors_options) = CorsOptions::default().to_cors() else {
        std::process::exit(0)
    };

    rocket::build()
        .attach(Template::fairing())
        .attach(cors_options)
        .manage(handler_database)
        .mount("/chaves", chaves::rotas())
}
```