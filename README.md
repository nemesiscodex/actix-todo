# actix-todo

A simple TODO list API built with Rust and Actix-web.

> **Note**: This version is updated for Rust 1.87. The original code can be found in the [`original-code`](https://github.com/nemesiscodex/actix-todo/tree/original-code) branch.

## Quick Start

1. **Setup environment**
   ```bash
   cp .env.example .env
   ```

2. **Start database**
   ```bash
   docker-compose up -d postgres
   ```

3. **Install dependencies**
   ```bash
   cargo install diesel_cli --no-default-features --features postgres
   ```

4. **Setup database**
   ```bash
   DATABASE_URL=postgres://actix:actix@localhost:5432/actix diesel migration run
   ```

5. **Run the server**
   ```bash
   cargo run
   ```

The API will be available at `http://localhost:8080`

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Health check |
| `GET` | `/todos` | List all todo lists |
| `GET` | `/todos/{id}` | Get a specific todo list |
| `POST` | `/todos` | Create a new todo list |
| `GET` | `/todos/{id}/items` | Get items in a todo list |
| `GET` | `/todos/{id}/items/{item_id}` | Get a specific item |
| `POST` | `/todos/{id}/items` | Add item to a todo list |
| `PUT` | `/todos/{id}/items/{item_id}` | Toggle item completion |

## Development

```bash
# Run tests
cargo test

# Run integration tests
cargo test --features "integration"
```

## Requirements

- Rust
- Docker & docker-compose
- PostgreSQL
