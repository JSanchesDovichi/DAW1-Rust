# Criar um novo projeto:
```
cargo new NOME_PROJETO
```

### Adicionar dependências ao projeto:
* No arquivo Cargo.toml, adicionar na seção [dependencies]:
```
rocket = { version = "0.5.0-rc.3", features = ["json"] }
rocket_dyn_templates = { version = "0.1.0-rc.3", features = ["tera"] }
mongodb = "2.6.1"
serde = "1.0.188"
```

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