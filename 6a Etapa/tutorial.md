### Adicionar função para inserir chave no CRUD de chaves:
(No arquivo dao/chaves.rs)
```
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
            },
            Err(e) => {
                println!("Erro ao criar/atualizar chave: {e}");

            }
        }

        None
    }
```

### Template da página de criação de chaves:
(Crie o arquivo paginas/criar_chave.html.tera)
```
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
    <h2 id="id_cadastro_de_chaves">CADASTRO DE CHAVE</h2>
    <input type="text" name="nomeDaChave" id="id_nome_da_chave" placeholder="Nome da chave">
    <button id="id_botao_enviar" type="button" onclick="funcaoDeClick()">ENVIAR</button>

    <script>
        function funcaoDeClick() {
            var myInput = document.getElementById("id_nome_da_chave");
            if (myInput && myInput.value) {

                const dataObject = {
                    nome: myInput.value,
                };

                fetch('http://127.0.0.1:8080/chaves', {
                    method: 'PUT',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(dataObject)
                }).then(response => {
                    //return respose.json()
                    if (response.status == 409) {
                        alert("Houve um erro ao criar a chave!")
                    } else  if (response.status == 200) {
                        alert("Chave criada com sucesso!")
                    }
                }).then(data => console.log(data));
            } else {
                alert("Escreva um nome antes de enviar!");
            }
        }

    </script>

</body>

</html>
```

### Adicionar endpoint de acesso para o template:
(No arquivo rotas/chaves.rs)
```
#[get("/nova")]
async fn pagina_criar_chave() -> Template {
    Template::render("criar_chave", context! {})
}



#[put("/", data = "<chave_para_criacao>")]
async fn criar_chave(database: &State<Database>, chave_para_criacao: Json<ChaveParaCriacao>) -> Status {
    let colecao_chaves = ColecaoChaves::default(database);

    if colecao_chaves.adicionar_chave(&chave_para_criacao.nome).await.is_some() {
        Status::Ok
    } else {
        Status::Conflict
    }
}
```