// File: src/models.rs
// High-level: Shared data models passed between layers and serialized to/from JSON.
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use slog;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub log: slog::Logger,
}

// Lightweight health response payload to keep `/` simple and cacheable.
#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

// Represents a row in `todo_item`; derives serde for JSON IO and PostgresMapper for row mapping.
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub checked: bool,
}

// Represents a row in `todo_list`.
#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

// Payload for creating a todo list; kept minimal on purpose.
#[derive(Serialize, Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

// Payload for creating a todo item.
#[derive(Serialize, Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
}

// Generic boolean result wrapper used by update endpoints.
#[derive(Serialize)]
pub struct ResultResponse {
    pub result: bool,
}
