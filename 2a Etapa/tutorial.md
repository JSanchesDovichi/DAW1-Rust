# Criar um novo projeto

Escolha uma pasta raiz onde o seu projeto ficará. Lembre-se de não se
esquecer dessa pasta. O Projeto será criado como uma subpasta dentro
dessa pasta.

```sh
cargo new NOME_PROJETO
```

O projeto terá a seguinte estrutura de arquivos:

```sh
.
├── Cargo.toml
└── src
    └── main.rs
```

Para os próximos passos, entre na pasta do projeto:

```sh
cd NOME_PROJETO
```

## Instalação de dependências

Todas as dependências do projeto são armazenadas no arquivo Cargo.toml.

### Framework Rocket

* No arquivo Cargo.toml, adicionar na seção [dependencies]:
* Link para documentação da framework: [Rocket](https://rocket.rs/v0.5-rc/guide/)

> ALERTA: Adicione apenas as linhas destacadas em verde. NÃO copie o "+".

```diff
[package]
name = "NOME_PROJETO"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
+   rocket = { version = "0.5.0-rc.3", features = ["json"] }
+   rocket_dyn_templates = { version = "0.1.0-rc.3", features = ["tera"] }
+   rocket_cors = { git = "https://github.com/lawliet89/rocket_cors", branch = "master" }
```

> Arquivo exemplo disponível em [Cargo.toml](../exemplos/etapa2/Cargo.toml)

### Dependência MongoDB

* No arquivo Cargo.toml, adicionar na seção [dependencies]:
* Link para documentação da dependência: [MongoDB](https://docs.rs/mongodb/2.6.1/mongodb/)

```diff
[package]
name = "NOME_PROJETO"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rocket = { version = "0.5.0-rc.3", features = ["json"] }
rocket_dyn_templates = { version = "0.1.0-rc.3", features = ["tera"] }
rocket_cors = { git = "https://github.com/lawliet89/rocket_cors", branch = "master" }
+ mongodb = "2.6.1"
```

> Arquivo exemplo disponível em [Cargo.toml](../exemplos/etapa2/Cargo.toml)

## Adicionar código para iniciar a API

> ALERTA: Adicione apenas as linhas destacadas em VERDE. NÃO copie o "+".
> ALERTA: Remova apenas as linhas destacadas em VERMELHO.

* No arquivo src/main.rs:

```diff
+ use rocket::launch;

- fn main() {
-     println!("Hello, world!");
- }

+ #[launch]
+ async fn rocket() -> _ {
+     rocket::build()
+ }
```

> Arquivo exemplo disponível em [main.rs](../exemplos/etapa2/src/main.rs)

## Mudar porta do servidor

* Crie o arquivo Rocket.toml (no mesmo diretório do Cargo.toml):
  
```diff
   .
   ├── Cargo.toml
+  ├── Rocket.toml
   └── src
      └── main.rs

```

* Adicione os campos dentro do arquivo:

```toml
[default]
address = "0.0.0.0"
port = 8080
template_dir = "src/paginas"
```

* Neste momento, é uma boa ideia tentar executar a API, para ver se está tudo funcionando,
  além de baixar as dependências necessárias.

> O comando deve ser executado na mesma pasta em que se encontra o arquivo Cargo.toml

```sh
cargo run
```

Se nenhum erro ocorrer, o terminal irá mostrar informações do servidor:

```sh
Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/NOME_PROJETO`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8080
   >> workers: 12
   >> max blocking threads: 512
   >> ident: Rocket
   >> IP header: X-Real-IP
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /tmp
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📡 Fairings:
   >> Shield (liftoff, response, singleton)
🛡 Shield:
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
🚀 Rocket has launched from http://127.0.0.1:8080
```

Após o comando terminar, o servidor será iniciado, e estará acessível em <http://127.0.0.1:8080>

IMPORTANTE: Os próximos passos envolvem mudanças no código fonte do projeto. Portante, será necessário recompilar o código.

Para compilar o código, é obrigatório estar na mesma pasta em que o projeto foi criado, no contexto desse tutorial, utilizamos a pasta "NOME_PROJETO".

* Para interromper o funcionamento da API, basta utilizar a combinação de teclas Ctrl+C no terminal em que ela esteja rodando:
  
<kbd>Ctrl</kbd> + <kbd>C</kbd>
  
* Para apenas compilar o projeto e não executar a API, basta utilizar o comando:
  
```sh
cargo build
```

* Para compilar e executar a API automaticamente, basta utilizar o comando:
  
```sh
cargo run
```
