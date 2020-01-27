use crate::models::{TodoList, TodoItem};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, io::Error> {
    let statement = client.prepare("insert into todo_list (title) values ($1) returning id, title").await.unwrap();

    client.query(&statement, &[&title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo list"))

}

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client.prepare("select * from todo_list order by id desc").await.unwrap();

    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todos)

}

pub async fn get_todo(client: &Client, list_id: i32) -> Result<TodoList, io::Error> {

    let statement = client.prepare("select * from todo_list where id = $1").await.unwrap();

    let maybe_todo = client.query_opt(&statement, &[&list_id])
        .await
        .expect("Error getting todo lists")
        .map(|row| TodoList::from_row_ref(&row).unwrap());

    match maybe_todo {
        Some(todo) => Ok(todo),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found"))
    }
}

pub async fn create_item(client: &Client, list_id: i32, title: String) -> Result<TodoItem, io::Error> {
    let statement = client.prepare("insert into todo_item (list_id, title) values ($1, $2) returning id, list_id, title, checked").await.unwrap();

    client.query(&statement, &[&list_id, &title])
        .await
        .expect("Error creating todo list")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>()
        .pop()
        .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating todo item"))

}



pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client.prepare("select * from todo_item where list_id = $1 order by id").await.unwrap();
    
    let items = client.query(&statement, &[&list_id])
        .await
        .expect("Error getting todo lists")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)

}

pub async fn get_item(client: &Client, list_id: i32, item_id: i32) -> Result<TodoItem, io::Error> {

    let statement = client.prepare("select * from todo_item where list_id = $1 and id = $2").await.unwrap();

    let maybe_item = client.query_opt(&statement, &[&list_id, &item_id])
        .await
        .expect("Error getting todo lists")
        .map(|row| TodoItem::from_row_ref(&row).unwrap());

    match maybe_item {
        Some(item) => Ok(item),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found"))
    }
}

pub async fn check_todo(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {

    let statement = client.prepare("update todo_item set checked = true where list_id = $1 and id = $2 and checked = false").await.unwrap();

    let result = client.execute(&statement, &[&list_id, &item_id])
        .await
        .expect("Error getting todo lists");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list"))
    }

}