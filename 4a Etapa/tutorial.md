# Instalação do banco de dados

* Instale o plugin para o docker compose utilizando o seguinte comando:
  
```sh
sudo apt-get install docker-compose-plugin
```

* Crie uma pasta em qualquer lugar da sua máquina com o nome de DAW1-Rust. É importantíssimo que ela tenha o nome de `DAW1-Rust`.
* Crie o arquivo docker-compose.yaml dentro dessa pasta.
* O seguinte código deve ser colocado dentro do arquivo docker-compose.yaml
  
```yaml
# Use root/example as user/password credentials
version: '3.1'

services:
  mongo:
    image: mongo
    restart: always
    environment:
      MONGO_INITDB_ROOT_USERNAME: root
      MONGO_INITDB_ROOT_PASSWORD: example
    ports:
      - 27017:27017

  mongo-express:
    image: mongo-express
    restart: always
    ports:
      - 8081:8081
    environment:
      ME_CONFIG_MONGODB_ADMINUSERNAME: root
      ME_CONFIG_MONGODB_ADMINPASSWORD: example
      ME_CONFIG_MONGODB_URL: mongodb://root:example@mongo:27017/

```

* Agora use o terminal para navegar até a localização desse arquivo, e execute o seguinte comando:
  
```sh
sudo docker compose up -d
```

* Para verificar se o contâiner do banco de dados foi iniciado corretamente, utilize o comando
  
```sh
sudo docker ps
```

A saída do comando deve conter pelo menos os dois containeres a seguir:

```sh
CONTAINER ID   IMAGE           COMMAND                  CREATED         STATUS         PORTS                                           NAMES
ad052edfa8a8   mongo           "docker-entrypoint.s…"   5 seconds ago   Up 4 seconds   0.0.0.0:27017->27017/tcp, :::27017->27017/tcp   daw1-rust-mongo-1
e2d4e5660f0a   mongo-express   "/sbin/tini -- /dock…"   5 seconds ago   Up 4 seconds   0.0.0.0:8081->8081/tcp, :::8081->8081/tcp       daw1-rust-mongo-express-1
```

* Crie o arquivo conexao.rs na pasta src do projeto:

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
+  ├── conexao.rs
    ├── enums.rs
    └── main.rs
```

* Adicione o seguinte código no arquivo conexao.rs:

```rust
use mongodb::Database;
use mongodb::options::Credential;
use mongodb::{Client, options::ClientOptions};
use mongodb::error::Result;

pub async fn get_database() -> Result<Database> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    client_options.credential = Some(Credential::builder()
    .username("root".to_string())
    .password("example".to_string()).build());

    match Client::with_options(client_options) {
        Ok(cliente) => {
            let db = cliente.database("daw1-rust");

            return Ok(db);
        },
        Err(e) => {
            return Err(e);
        }
    }
}
```

* Adicione o código de conexão no arquivo main.rs:

```diff
use rocket::launch;

pub mod classes;
pub mod enums;
+ pub mod conexao;

#[launch]
async fn rocket() -> _ {
+    let Ok(database_handler) = conexao::get_database().await else {
+        panic!("Não foi possivel iniciar a conexão com o banco de dados!")
+    };

    rocket::build()
+    .manage(database_handler)
}
```
