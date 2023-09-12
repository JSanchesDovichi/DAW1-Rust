# Criar um novo projeto:
```
cargo new NOME_PROJETO
```

### Instalação da framework Rocket:
* No arquivo Cargo.toml, adicionar na seção [dependencies]:
```
rocket = { version = "0.5.0-rc.3", features = ["json"] }
rocket_dyn_templates = { version = "0.1.0-rc.3", features = ["tera"] }
```
* Link para documentação da framework: [Rocket](https://rocket.rs/v0.5-rc/guide/)

### Instalação de outras dependências necessárias:
* No arquivo Cargo.toml, adicionar na seção [dependencies]:
```
mongodb = "2.6.1"
```
* Link para documentação da dependência: [MongoDB](https://docs.rs/mongodb/2.6.1/mongodb/)

### Mudar porta do servidor:
* Crie o arquivo Rocket.toml (no mesmo diretório do Cargo.toml), e adicione os campos:
```
[default]
address = "0.0.0.0"
port = 8080
```

* Neste momento, é uma boa ideia tentar executar a API, para ver se está tudo funcionando,
  além de baixar as dependências necessárias:
```
cargo run
```