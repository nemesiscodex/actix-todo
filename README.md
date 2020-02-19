# actix-todo
Simple TODO list API made in rust

## Requirements
- Rust
- Docker
- docker-compose

## Usage
```
docker-compose up -d postgres
cargo run --release
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
