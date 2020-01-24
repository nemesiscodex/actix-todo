use crate::models::{TodoList, TodoItem};
use tokio_postgres::{Client, connect, NoTls, Statement, Error};
use futures::{FutureExt, StreamExt};
use actix::{Actor, Context, Addr, Message, Handler, ResponseFuture};
use std::io;
use std::boxed::Box;

pub struct PgConnection {
    cl: Client,
    todos: Statement,
    todo: Statement
}

impl Actor for PgConnection {
    type Context = Context<Self>;
}

impl PgConnection {
    pub async fn connect(db_url: &str) -> Result<Addr<PgConnection>, io::Error> { 
        let (cl, conn) = connect(db_url, NoTls)
            .await
            .expect("Cannot connect to postgresql");

        actix_rt::spawn(conn.map(|_| ()));

        let todos = cl.prepare("select id, title from todo_list")
            .await
            .expect("Cannot create prepared statement");

        let todo = cl.prepare("select id, title from todo_list where id = $1")
            .await
            .expect("Cannot create prepared statement");

        Ok(PgConnection::create(move |_| PgConnection{cl, todos, todo}))
        
    }
}

pub struct GetTodoLists;

impl Message for GetTodoLists {
    type Result = io::Result<Vec<TodoList>>;
}

fn handle_error(e: Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{:?}", e))
}

impl Handler<GetTodoLists> for PgConnection {
    type Result = ResponseFuture<Result<Vec<TodoList>, io::Error>>;

    fn handle(&mut self, _: GetTodoLists, _: &mut Self::Context) -> Self::Result { 
        let mut result: Vec<TodoList> = vec![];

        let fut = self.cl.query_raw(&self.todos, &[]);

        Box::pin(async move {

            let mut stream = fut
            .await
            .map_err(handle_error)?;

            
            while let Some(row) = stream.next().await {
                let row = row
                    .map_err(handle_error)?; 

                result.push(TodoList{
                    id: row.get(0),
                    title: row.get(1)
                });
            }

            Ok(result)

        })
        
    }
}

pub struct GetTodoList {
    pub id: i32
}

impl Message for GetTodoList {
    type Result = io::Result<TodoList>;
}

impl Handler<GetTodoList> for PgConnection {
    type Result = ResponseFuture<Result<TodoList, io::Error>>;

    fn handle(&mut self, msg: GetTodoList, _: &mut Self::Context) -> Self::Result { 

        let fut = self.cl.query_one(&self.todo, &[&msg.id]);

        Box::pin(async move {

            let row = fut
            .await
            .map_err(handle_error)?;

            Ok(TodoList {
                id: row.get(0),
                title: row.get(1)
            })

        })
        
    }
}

