use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub title: String,
    pub checked: bool
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub id: i32,
    pub title: String
}