# Discord Bot Template (Rust) 🦀

Um template robusto e moderno para criação de bots de Discord utilizando **Rust**, **Poise** (sobre **Serenity**) e as melhores práticas de desenvolvimento. Inspirado no [discord-bot-template](../discord-bot) em TypeScript.

## ✨ Funcionalidades

- 🦀 **Rust + Tokio**: tipagem forte, segurança de memória e performance.
- ⚡ **Slash Commands com Poise**: comandos declarados com `#[poise::command]`, organizados por categoria.
- 📂 **Organização modular**: cada categoria de comando e cada evento vive no seu próprio módulo.
- 📝 **Logs com `tracing`**: console colorido e arquivos em `logs/` (`combined.log` e `error.log`).
- 🎨 **Qualidade de código**: `rustfmt` (4 espaços) e `clippy` com `-D warnings`, validados no CI.
- ⚙️ **Gestão de variáveis**: `.env` via `dotenvy`, validado na inicialização com erro claro quando falta alguma variável.
- 🔐 **Checks de permissão**: exemplo de comando restrito ao dono do bot (`/set-status`).

## 🛠️ Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, Rust 1.85 ou superior)
- Uma conta no [Discord Developer Portal](https://discord.com/developers/applications)

## 🚀 Instalação e Início

1.  **Clone o repositório:**

    ```bash
    git clone https://github.com/marcuthus/discord-bot-rs.git
    cd discord-bot-rs
    ```

2.  **Configure o ambiente:**
    Copie o arquivo `.env.example` para `.env` e preencha as variáveis:

    ```env
    DISCORD_BOT_TOKEN=seu_token_aqui
    DISCORD_ADMIN_ID=seu_id_de_usuario_discord
    DISCORD_APPLICATION_ID=id_da_sua_aplicacao
    DISCORD_PUBLIC_KEY=sua_chave_publica
    ```

3.  **Inicie o bot:**

    ```bash
    # Modo desenvolvimento
    cargo run

    # Build para produção
    cargo build --release
    ./target/release/discord-bot-rs
    ```

## 📜 Comandos Úteis

| Comando                                       | Descrição                                 |
| --------------------------------------------- | ----------------------------------------- |
| `cargo run`                                   | Executa o bot em modo de desenvolvimento. |
| `cargo build --release`                       | Compila o bot otimizado para produção.    |
| `cargo fmt`                                   | Formata o código.                         |
| `cargo clippy --all-targets -- -D warnings`   | Verifica erros de linting.                |
| `cargo test`                                  | Executa os testes.                        |

Defina `RUST_LOG=debug` para ver logs mais detalhados no console.

## 📁 Estrutura de Pastas

```text
src/
├── main.rs          # Ponto de entrada
├── config.rs        # Variáveis de ambiente (Config)
├── logger.rs        # Configuração do tracing (console + arquivos)
├── client/          # Montagem do framework Poise
│   ├── data.rs      # Data, Context e Error compartilhados
│   └── error.rs     # Handler global de erros
├── commands/        # Comandos (Slash Commands) por categoria
│   ├── info/
│   └── admin/
├── events/          # Handlers de eventos do Discord
└── utils/           # Utilitários (checks de permissão, etc.)
```

## ➕ Adicionando um comando

1. Crie `src/commands/<categoria>/<comando>.rs` com uma função `#[poise::command(slash_command)]`.
2. Declare o módulo e inclua a função em `commands()` no `mod.rs` da categoria.
3. Se for uma categoria nova, inclua-a em `commands::all()` (`src/commands/mod.rs`).

> Diferente do template em TypeScript, Rust não carrega módulos em tempo de execução, então o "auto-loading" é feito por esses `mod.rs`.

## ➕ Tratando um novo evento

Crie `src/events/<evento>.rs` e adicione um braço ao `match` em `events::handle` (`src/events/mod.rs`).

## ⚖️ Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

Feito com ❤️ por [Marcuth](https://github.com/marcuth)
