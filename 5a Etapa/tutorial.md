# Implementando Endpoints:

# Listar coleções do banco de dados:
* Endpoint teste para receber uma lista de strings em formato json:
```
use rocket::{get, serde::json::Json};

#[get("/lista_strings")]
async fn listar_string_json() -> Json<Vec<String>>{
    let lista_strings = vec![
        "teste1".to_string(),
        "teste2".to_string(),
        "teste3".to_string()
    ];

    Json(lista_strings)
}
```