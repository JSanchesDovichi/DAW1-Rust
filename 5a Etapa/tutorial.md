### Função CRUD para listar as chaves disponíveis no sistema:
(dao/chave.rs)
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
(Crie o arquivo paginas/chaves.html.tera)
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

### Endpoint de acesso para o template:
(No arquivo rotas/chaves.rs)
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
        buscar_chave,
        criar_chave,
        remover_chave,
        atualizar_chave
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

### Adicionando a rota na api:
(No arquivo main.rs)
```
use rocket::{launch};
use rocket_dyn_templates::Template;

mod conexao;
mod paginas;
mod classes;

mod rotas;

mod dao;

use rotas::{chaves, servidor};

#[launch]
async fn rocket() -> _ {
    let Some(handler_database) = conexao::get_database().await else {
        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
    };

    rocket::build()
        .attach(Template::fairing())
        .manage(handler_database)
        .mount("/", paginas::rotas())
        .mount("/chaves", chaves::rotas())
        .mount("/servidores", servidor::rotas())
}
```