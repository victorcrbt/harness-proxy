# Harness Proxy (Intent Router)

Middleware em Rust que gerencia e roteia automaticamente requisições de Inteligência Artificial entre as IDEs e o Gateway LiteLLM.

## Pré-requisitos
* [Rust](https://rustup.rs/) instalado localmente.

## Configuração do Ambiente
Crie um arquivo `.env` na raiz do projeto (ou exporte a variável no sistema) para definir o endereço do gateway alvo:
```env
GATEWAY_URL=<URL do Gateway de Roteamento>
```
*Caso omitido, o sistema adotará o IP padrão acima.*

## Como Instalar e Rodar
1. **Para desenvolver e testar na sua máquina**:
   ```bash
   cargo run
   ```
2. **Para compilar o binário final (produção)**:
   ```bash
   cargo build --release
   ```
   *O executável otimizado estará em `./target/release/harness-proxy`. Mova esse arquivo para o servidor.*

## Integração (Kilo Code / Open WebUI)
Aponte as configurações das ferramentas da seguinte forma:
* **Base URL:** `http://localhost:11436/v1` (se estiver na mesma máquina) ou `http://IP_DO_SERVIDOR:11436/v1` (se o proxy estiver no servidor remoto).
