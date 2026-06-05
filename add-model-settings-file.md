# Tarefa: Configuração Dinâmica de Modelos e Hot Reload

**Contexto Geral para o Agente:**
O objetivo desta tarefa é desacoplar a lista de modelos da compilação do servidor Rust. A lista deve ser lida a partir de um arquivo `models.json` em tempo de execução. O modelo `"Auto (Local)"` é uma regra de negócio fixa e deve sempre ser injetado no topo do retorno da API. As alterações no arquivo JSON devem refletir imediatamente na API sem a necessidade de reiniciar o servidor (Hot Reload).

---

## PARTE 1: Implementação

### Passo 1.1: Criar o Arquivo Base de Modelos
Crie o arquivo de configuração na raiz do projeto. O modelo "Auto" não entra aqui, pois é fixo no código.

**Caminho:** `models.json`
**Conteúdo:**
```json
[
  "openrouter/deepseek/deepseek-v4-pro",
  "openrouter/openai/o1",
  "openrouter/openai/o3-mini",
  "openrouter/anthropic/claude-sonnet-4.5",
  "openrouter/deepseek/deepseek-v4-flash",
  "openrouter/qwen/qwen3-coder-plus",
  "openrouter/google/gemini-2.5-pro",
  "openrouter/moonshotai/kimi-k2.5",
  "openrouter/meta-llama/llama-3-70b-instruct",
  "openrouter/anthropic/claude-haiku-4.5",
  "openrouter/mistralai/mistral-large-2512",
  "openrouter/openai/gpt-4o",
  "openrouter/qwen/qwen-vl-plus",
  "openrouter/google/gemini-2.5-flash",
  "openrouter/z-ai/glm-4.5-air:free",
  "deepseek-r1-local"
]
```

### Passo 1.2: Refatorar o Handler de Listagem
Substitua o conteúdo da função que responde pela rota `GET /v1/models` (localizada em `src/handlers/models.rs` ou no arquivo equivalente) pelo código abaixo.

**Código:**
```rust
use axum::Json;
use serde_json::{json, Value};
use std::fs;

pub async fn list_models() -> Json<Value> {
    // 1. O Auto (Local) é fixo e sempre o primeiro a ser injetado
    let mut data = vec![json!({
        "id": "Auto (Local)",
        "object": "model",
        "created": 1686935002,
        "owned_by": "harness"
    })];

    // 2. Leitura assíncrona a cada requisição (Garante o Hot Reload)
    if let Ok(content) = fs::read_to_string("models.json") {
        if let Ok(models) = serde_json::from_str::<Vec<String>>(&content) {
            for model_id in models {
                data.push(json!({
                    "id": model_id,
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "harness-config"
                }));
            }
        } else {
            eprintln!("Aviso: Falha de parsing no models.json. O arquivo não contém um array de strings válido.");
        }
    } else {
        eprintln!("Aviso: Arquivo models.json não encontrado no diretório de execução. Fallback ativo.");
    }

    // 3. Montagem da estrutura de resposta no padrão exigido pela IDE
    Json(json!({
        "object": "list",
        "data": data
    }))
}
```

---

## PARTE 2: Plano de Testes (Validação do Agente)

**Pré-requisitos:** O servidor Rust deve estar em execução em background (ex: rodando `cargo run`). Utilize o terminal secundário para testar.

### Teste 1: Validação de Leitura Inicial
Verifica se o servidor consegue ler o JSON recém-criado e fundir com o modelo estático.

**Ação:**
```bash
curl -s [http://127.0.0.1:11436/v1/models](http://127.0.0.1:11436/v1/models)
```
**Critério de Sucesso:**
* O HTTP status deve ser 200.
* A resposta JSON deve conter `"Auto (Local)"` no início do array `"data"`, seguido imediatamente por `"openrouter/deepseek/deepseek-v4-pro"` e todos os outros declarados no arquivo.

### Teste 2: Validação de Hot Reload
Verifica se alterações no arquivo refletem na API sem reiniciar o processo.

**Ação:**
1. Abra o arquivo `models.json`.
2. Adicione a string `"modelo-teste-hot-reload"` ao final do array e salve o arquivo. **NÃO reinicie o servidor Rust**.
3. Execute novamente:
```bash
curl -s [http://127.0.0.1:11436/v1/models](http://127.0.0.1:11436/v1/models)
```
**Critério de Sucesso:**
* A string `"modelo-teste-hot-reload"` deve aparecer na resposta JSON instantaneamente. Se for necessário reiniciar o servidor para ela aparecer, o teste falhou.

### Teste 3: Tolerância a Falhas (Fallback)
Verifica se o servidor sobrevive à ausência ou exclusão acidental do arquivo de configuração.

**Ação:**
1. Mova ou apague o arquivo temporariamente: `mv models.json models_bkp.json`.
2. Execute a chamada novamente:
```bash
curl -s [http://127.0.0.1:11436/v1/models](http://127.0.0.1:11436/v1/models)
```
**Critério de Sucesso:**
* A API **NÃO** deve retornar erro 500 ou fechar a conexão.
* O terminal onde o Rust está rodando deve imprimir o erro silencioso (`Aviso: Arquivo models.json não encontrado...`).
* A resposta JSON deve retornar com sucesso contendo **apenas** o modelo `"Auto (Local)"` no array `"data"`.