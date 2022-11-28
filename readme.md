# API com Rust - Teste
Projeto de estudo da linguagem rust com o pacote actix-web para desenvolvimento de api.

## Build
- ambiente de desenvolvimento: `cargo build`
- ambiente de produção: `cargo build --release`

## Run
- ambiente de desenvolvimento: `cargo run`
- ambiente de produção: `./target/release/rust-api`

## Docker
- `docker build --tag rust-api .`
- `docker run -d -p 8080:8080 --name rust-api rust-api`