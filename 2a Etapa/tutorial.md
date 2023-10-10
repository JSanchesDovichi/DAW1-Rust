# Criar um novo projeto:
```
cargo new NOME_PROJETO
```

### Instalação da framework Rocket:
* No arquivo Cargo.toml, adicionar na seção [dependencies]:
```
rocket = { version = "0.5.0-rc.3", features = ["json"] }
rocket_dyn_templates = { version = "0.1.0-rc.3", features = ["tera"] }
rocket_cors = { git = "https://github.com/lawliet89/rocket_cors", branch = "master" }
```
* Link para documentação da framework: [Rocket](https://rocket.rs/v0.5-rc/guide/)

### Instalação de outras dependências necessárias:
* No arquivo Cargo.toml, adicionar na seção [dependencies]:
```
mongodb = "2.6.1"
```
* Link para documentação da dependência: [MongoDB](https://docs.rs/mongodb/2.6.1/mongodb/)

### Adicionar código para iniciar a API:
(No arquivo main.rs)
```
use rocket::launch;
use rocket_cors::CorsOptions;
use rocket_dyn_templates::Template;

mod conexao;

#[launch]
async fn rocket() -> _ {
    let Some(handler_database) = conexao::get_database().await else {
        panic!("Não foi possível iniciar a conexão com o banco de dados!")
    };

    let Ok(cors_options) = CorsOptions::default().to_cors() else {
        std::process::exit(0)
    };

    rocket::build()
    
}
```

### Arquivo para conexão com banco de dados:
Crie o arquivo src/conexao.rs
```
use mongodb::Database;
use mongodb::options::Credential;
use mongodb::{Client, options::ClientOptions};

pub async fn get_database() -> Option<Database> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();

    client_options.credential = Some(Credential::builder()
    .username("root".to_string())
    .password("example".to_string()).build());

    if let Ok(cliente) = Client::with_options(client_options) {
        let db = cliente.database("daw1-rust");

        Some(db)
    } else {
        None
    }

}
```

### Mudar porta do servidor:
* Crie o arquivo Rocket.toml (no mesmo diretório do Cargo.toml), e adicione os campos:
```
[default]
address = "0.0.0.0"
port = 8080
template_dir = "paginas"
```

* Neste momento, é uma boa ideia tentar executar a API, para ver se está tudo funcionando,
  além de baixar as dependências necessárias:
```
cargo run
```