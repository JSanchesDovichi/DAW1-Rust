# Pre-requisitos:

### Instalar Docker Compose (se não existir): 
```
sudo apt install docker-compose
```

### Banco de dados:
```
sudo docker-compose up -d
```

### Dependência build-essential:
```
sudo apt install build-essential
```

### Linguagem de programação Rust:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
* Durante a instalação: pressionar 1 quando requisitado para instalação padrão.

# Criar um novo projeto:
```
cargo new NOME_PROJETO
```

### Adicionar dependências ao projeto:
* No arquivo Cargo.toml, adicionar na seção [dependencies]:
```
rocket = { version = "0.5.0-rc.3", features = ["json"] }
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