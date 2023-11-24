# Editar o arquivo de classes para chaves

* Edite o arquivo de src/classes/chave.rs

```diff
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

-    #[serde(skip_deserializing)]
    pub situacao: EstadoChave,

    pub ativo: bool
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ChaveParaCriacao {
    pub nome: String,
}

+#[derive(Deserialize, Serialize)]
+#[serde(crate = "rocket::serde")]
+pub struct ChaveParaEdicao {
+   pub nome_original: String,
+   pub nome_novo: String,
+   pub estado_novo: EstadoChave
+}
```

* Edite o arquivo de src/dao/chave.rs
  
```diff
use crate::classes::chave::Chave;
use crate::enums::EstadoChave;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateOptions;
use mongodb::{Collection, Database};
use rocket::futures::StreamExt;
-use rocket::http::Status;
use rocket::log::private::error;
-use rocket::serde::json::Json;
use rocket::State;
-use std::str::FromStr;

pub struct ColecaoChaves {
    pub colecao: Collection<Chave>,
}

impl ColecaoChaves {
    pub fn default(database: &State<Database>) -> Self {
        ColecaoChaves {
            colecao: database.collection("Chaves"),
        }
    }

    pub async fn listar_chaves(&self) -> Vec<Chave> {
-        let filtro = doc![];
+        let filtro = doc![
+            "ativo": true
+        ];

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

    pub async fn adicionar_chave(&self, nome_chave_nova: &str) -> Option<ObjectId> {
        let filtro = doc![
            "nome": &nome_chave_nova
        ];

        let atualizacao = doc![
            "$set": doc![
                "situacao": EstadoChave::Disponivel,
                "ativo": true
            ]
        ];

        let opcoes = UpdateOptions::builder().upsert(true).build();

        match self.colecao.update_one(filtro, atualizacao, opcoes).await {
            Ok(resultado) => {
                if let Some(id_atualizaco) = resultado.upserted_id {
                    return id_atualizaco.as_object_id();
                }
            }
            Err(e) => {
                println!("Erro ao criar/atualizar chave: {e}");
            }
        }

        None
    }
+    pub async fn alterar_chave(&self, nome_original_chave: &String, nome_novo_chave: &String, estado_novo_chave: String) -> bool {
+        let filtro = doc![
+            "nome": &nome_original_chave
+        ];
+
+        let atualizacao = doc![
+            "$set": doc![
+                "nome": nome_novo_chave,
+                "situacao": estado_novo_chave,
+            ]
+        ];
+
+        match self.colecao.update_one(filtro, atualizacao, None).await {
+            Ok(resultado) => {
+                if resultado.modified_count == 1 {
+                    return true;
+                }
+            }
+            Err(e) => {
+                println!("Erro ao criar/atualizar chave: {e}");
+            }
+        }
+
+        false
+    }
+
+    pub async fn remover_chave(&self, nome_chave: String) -> bool {
+        let filtro = doc![
+            "nome": nome_chave
+        ];
+
+        let atualizacao = doc![
+            "$set": doc![
+                "ativo": false,
+            ]
+        ];
+
+        match self.colecao.update_one(filtro, atualizacao, None).await {
+            Ok(resultado) => {
+                if resultado.modified_count == 1 {
+                    return true;
+                }
+            }
+            Err(e) => {
+                println!("Erro ao remover chave: {e}");
+            }
+        }
+
+        false
+    }
}
```

* Crie um template novo para a página de edição de chaves em src/paginas/editar_chave.html.tera com o seguinte código:

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Chaves RUST</title>
    <style>
        /* Estilo para o corpo da página */
        body {
            background-color: #121212;
            font-family: 'Arial', sans-serif;
            margin: 0;
            padding: 0;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
        }

        /* Estilo para o cabeçalho h2 */
        h2#id_cadastro_de_chaves {
            font-size: 3em;
            /* 48px em */
            color: #fff;
            text-align: center;
            margin-bottom: 1.5em;
            /* 30px em */
            text-shadow: 0.1em 0.1em 0.2em rgba(0, 0, 0, 0.3);
        }

        /* Estilo para o input de texto */
        input[type="text"] {
            width: 18.75em;
            /* 300px em */
            padding: 1.25em;
            /* 20px em */
            margin: 1.25em 0;
            /* 20px em */
            border: 0.125em solid #ff6a00;
            /* 2px em */
            border-radius: 0.75em;
            /* 12px em */
            font-size: 1.5em;
            /* 24px em */
            background-color: #1a1a1a;
            color: #fff;
            outline: none;
            transition: border-color 0.3s ease, box-shadow 0.3s ease;
        }

        /* Estilo para o input de texto ao receber foco */
        input[type="text"]:focus {
            border-color: #ff4500;
            box-shadow: 0 0 1.25em rgba(255, 106, 0, 0.7);
            /* 20px em */
        }

        select {
            width: 18.75em;
            /* 300px em */
            padding: 1.25em;
            /* 20px em */
            margin: 1.25em 0;
            /* 20px em */
            border: 0.125em solid #ff6a00;
            /* 2px em */
            border-radius: 0.75em;
            /* 12px em */
            font-size: 1.5em;
            /* 24px em */
            background-color: #1a1a1a;
            color: #fff;
            outline: none;
            transition: border-color 0.3s ease, box-shadow 0.3s ease;
        }

        select:focus {
            border-color: #ff4500;
            box-shadow: 0 0 1.25em rgba(255, 106, 0, 0.7);
            /* 20px em */
        }

        /* Estilo para o botão */
        button#id_botao_enviar {
            padding: 1.25em 2.5em;
            /* 20px em */
            background-color: #ff6a00;
            color: #fff;
            border: none;
            border-radius: 0.75em;
            /* 12px em */
            font-size: 1.75em;
            /* 28px em */
            cursor: pointer;
            transition: background-color 0.3s ease, transform 0.3s ease;
        }

        /* Estilo para o botão ao passar o mouse sobre ele */
        button#id_botao_enviar:hover {
            background-color: #ff4500;
            transform: scale(1.05);
        }
    </style>
</head>

<body>
    <h2 id="id_cadastro_de_chaves">EDITAR CHAVE</h2>
    <input type="text" name="nomeDaChave" id="id_nome_da_chave" placeholder="{{ chave.nome }}">
    <select name="estados" id="id_estado">
        <option value="" selected="{{ chave.estado }}" hidden="hidden">{{ chave.estado }}</option>
        <option value="Disponivel">Disponivel</option>
        <option value="Emprestada">Emprestada</option>
    </select>
    <button id="id_botao_enviar" type="button" onclick="funcaoDeClick()">SALVAR</button>

    <script>
        function funcaoDeClick() {
            var nome_original_chave = "{{ chave.nome }}";
            var estado_original_chave = "{{ chave.estado }}";
            var myInput = document.getElementById("id_nome_da_chave");
            var selectInput = document.getElementById("id_estado");

            var dataObject = { 'placeholder': "placeholder" };

            estado_novo = selectInput.value;

            if(estado_novo == "") {
                estado_novo = estado_original_chave;
            }

            if (myInput && myInput.value) {
                dataObject = {
                    nome_original: nome_original_chave,
                    nome_novo: myInput.value,
                    estado_novo: estado_novo
                };
            } else {
                dataObject = {
                    nome_original: nome_original_chave,
                    nome_novo: nome_original_chave,
                    estado_novo: estado_novo
                };
            }

            fetch('http://127.0.0.1:8080/chaves/', {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(dataObject)
            }).then(response => {
                if (response.status == 409) {
                    alert("Houve um erro ao alterar a chave!")
                } else if (response.status == 200) {
                    alert("Chave alterada com sucesso!")
                }
            }).then(data => console.log(data));
        }

    </script>

</body>

</html>
```

* Substitua o arquivo src/rotas/chaves.rs com o seguinte código:

```rust
use crate::classes::chave::ChaveParaCriacao;
use crate::classes::chave::ChaveParaEdicao;
use crate::dao::chave::ColecaoChaves;
use mongodb::bson::doc;
use mongodb::Database;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, patch, put, routes, Route, State};
use rocket_dyn_templates::{context, Template};

pub fn rotas() -> Vec<Route> {
    routes![listar_chaves, pagina_criar_chave, criar_chave, pagina_editar_chave, editar_chave, remover_chave]
}

#[get("/")]
async fn listar_chaves(database: &State<Database>) -> Template {
    let lista_chaves = ColecaoChaves::default(database).listar_chaves().await;

    Template::render(
        "chaves",
        context! {
            chaves: lista_chaves
        },
    )
}

#[get("/editar?<name>&<situacao>")]
async fn pagina_editar_chave(name: &str, situacao: &str) -> Template {
    Template::render(
        "editar_chave",
        context! {
            chave: doc![
                "nome": name,
                "estado": situacao
            ]
        },
    )
}

#[get("/nova")]
async fn pagina_criar_chave() -> Template {
    Template::render("criar_chave", context! {})
}

#[patch("/", data = "<chave_para_edicao>")]
async fn editar_chave(
    database: &State<Database>,
    chave_para_edicao: Json<ChaveParaEdicao>,
) -> Status {

    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves
        .alterar_chave(&chave_para_edicao.nome_original, &chave_para_edicao.nome_novo, chave_para_edicao.estado_novo.to_string())
        .await
        == true
    {
        Status::Ok
    } else {
        Status::Conflict
    }
}

#[put("/", data = "<chave_para_criacao>")]
async fn criar_chave(
    database: &State<Database>,
    chave_para_criacao: Json<ChaveParaCriacao>,
) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves
        .adicionar_chave(&chave_para_criacao.nome)
        .await
        .is_some()
    {
        Status::Ok
    } else {
        Status::Conflict
    }
}

#[delete("/<nome_chave>")]
async fn remover_chave(
    database: &State<Database>,
    nome_chave: String
) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves
        .remover_chave(nome_chave)
        .await
        == true
    {
        Status::Ok
    } else {
        Status::Conflict
    }
}
```
