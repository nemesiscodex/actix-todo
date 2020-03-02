# actix-todo ![Build Status](https://github.com/nemesiscodex/actix-todo/workflows/tests/badge.svg)
Simple TODO list API made in rust

## Requirements
- Rust
- Docker
- docker-compose

## Usage
```
# Copy example .env file
cp .env.example .env

# Run postgres
docker-compose up -d postgres

# Install diesel
cargo install diesel_cli --no-default-features --features postgres

# Run db migrations
DATABASE_URL=postgres://actix:actix@localhost:5432/actix diesel migration run

# Run unit tests
cargo test

# Run integration tests
cargo test --features "integration"

# Run the server (Add --release for an optimized build)
cargo run 
```
```
curl -s http://localhost:8080/
```

# Routes

- `GET` `/` -> Status

  **Response:**
  ```
  {
    "status": "Up"
  }
  ```

- `GET` `/todos` -> Get todo lists

  **Response:**
  ```
  [
    {
      "id": 1,
      "title": "Grocery list"
    },
    ...
  [
  ```
- `GET` `/todos/1` -> Get single todo list

  **Response:**
  ```
  {
    "id": 1,
    "title": "Grocery list"
  }
  ```
- `POST` `/todos` -> Create todo list

  **Request Header:**
  ```
  Content-Type: application/json
  ```
  **Request Body:**
  ```
  {
    "title": "List title"    
  }
  ```
  **Response:**
  ```
  {
    "id": 1,
    "title": "Grocery list"
  }
  ```
- `GET` `/todos/1/items` -> Get items of the todo list

  **Response:**
  ```
  [
    {
      "id": 1,
      "list_id": 1,
      "title": "Milk",
      "checked": true
    },
    {
      "id": 1,
      "list_id": 2,
      "title": "Bread",
      "checked": false
    }
  ]
  ```
- `GET` `/todos/1/items/1` -> Get single item of the todo list

  **Response:**
  ```
  {
    "id": 1,
    "list_id": 1,
    "title": "Milk",
    "checked": true
  }
  ```

- `POST` `/todos/1/items` -> Create todo list item

  **Request Header:**
  ```
  Content-Type: application/json
  ```
  **Request Body:**
  ```
  {
    "title": "Eggs"    
  }
  ```
  **Response:**
  ```
  {
    "id": 1,
    "list_id": 1,
    "title": "Eggs",
    "checked": false
  }
  ```
- `PUT` `/todos/1/items/1` -> Check todo

  **Response:**
  ```
  {
    "result": true
  }
  ```
  Result:
  - `true` -> Checked
  - `false` -> Already checked. Nothing to do.
