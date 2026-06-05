# Harness Proxy (Intent Router)

Middleware em Rust que gerencia e roteia automaticamente requisições de IA entre IDEs e o Gateway LiteLLM.

## Pré-requisitos

- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org/) + [Yarn](https://yarnpkg.com/) (apenas para o painel web)

## Configuração

Crie um arquivo `.env` na raiz do projeto:

```env
GATEWAY_URL=<URL do Gateway LiteLLM>
```

*Fallback: `http://localhost:11435/v1`.*

---

## Como Rodar

### Servidor + Painel Web (recomendado)

```bash
cargo run --bin harness-proxy
```

- Proxy Axum: `http://127.0.0.1:11436`
- Painel React: `http://127.0.0.1:3528`
- `models.json` criado automaticamente na primeira execução

### Modo Desktop (Tauri)

```bash
cargo tauri dev
```

Abre janela nativa com o painel embutido.

---

## Painel de Gerenciamento (`http://localhost:3528`)

### Funcionalidades

| Ação | Como usar |
|---|---|
| **Adicionar** | Digite o ID e clique em **Adicionar** |
| **Editar** | Clique ✏️ — o nome vai para o input, botões mudam para **Salvar** 💾 / **Cancelar** ✕ |
| **Remover** | Clique 🗑️ |
| **Ordenar** | Clique em **A → Z** / **Z → A** no cabeçalho |
| **Buscar** | Digite no campo de pesquisa (suporta regex, ex: `gemini\|claude`) |
| **Paginar** | Controles no rodapé: seletor de itens/página (5/10/20/50) + navegação entre páginas |
| **Dark Mode** | Seletor no canto inferior da barra lateral |

### Exemplos de busca

```
gemini              → mostra modelos com "gemini" no nome
openai\|anthropic    → modelos da OpenAI e Anthropic
^openrouter          → modelos que começam com "openrouter"
4\.5$                → modelos que terminam com "4.5"
```

---

## Rotas da API (Axum)

| Método | Rota | Descrição |
|---|---|---|
| `GET` | `/v1/models` | Lista modelos para IDE |
| `POST` | `/v1/chat/completions` | Proxy de chat |
| `GET` | `/v1/config/models` | Lê `models.json` |
| `POST` | `/v1/config/models` | Sobrescreve `models.json` |
| `*` | `/v1/*` | Proxy transparente para o Gateway |

---

## Integração com IDEs

Aponte a Base URL da IDE para:

```
http://localhost:11436/v1
```

---

## Compilação para Produção

```bash
cargo build --release
# Binário em ./target/release/harness-proxy
```
