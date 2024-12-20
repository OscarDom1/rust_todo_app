use actix_web::{ error::InternalError, web, Error, HttpResponse, Result};
use mysql::{prelude::*, Pool, Row};
use serde::{Deserialize, Serialize};
use mysql::params;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl mysql::prelude::FromRow for Todo {
    fn from_row(row: Row) -> Self {
        Todo {
            id: row.get("id").unwrap_or(0),
            title: row.get("title").unwrap_or_default(),
            description: row.get("description").unwrap_or_default(),
            completed: row.get("completed").unwrap_or(false),
        }
    }
    
    fn from_row_opt(_row: Row) -> std::result::Result<Self, mysql::FromRowError>
    where
        Self: Sized {
        todo!()
    }
}


fn map_mysql_error(e: mysql::Error) -> actix_web::Error {
    // Return an InternalError with a string message instead of a HttpResponse
    InternalError::new(format!("Database error: {}", e), actix_web::http::StatusCode::INTERNAL_SERVER_ERROR).into()
}

// Function to get all todos
pub async fn get_todos(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get_conn().map_err(map_mysql_error)?;

    // Fetch todos from the database
    let todos: Vec<Todo> = conn
        .query_map(
            "SELECT id, title, description, completed FROM todos", 
            |(id, title, description, completed)| Todo { id, title, description, completed }
        )
        .map_err(map_mysql_error)?;

    Ok(HttpResponse::Ok().json(todos))
}

// Function to get a single todo by ID
pub async fn get_todo(pool: web::Data<Pool>, todo_id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get_conn().map_err(map_mysql_error)?;

    let todo: Option<Todo> = conn
        .exec_first(
            "SELECT id, title, description, completed FROM todos WHERE id = :id",
            params! { "id" => *todo_id },
        )
        .map_err(map_mysql_error)?;

    match todo {
        Some(todo) => Ok(HttpResponse::Ok().json(todo)),
        None => Ok(HttpResponse::NotFound().json("Todo not found")),
    }
}

// Function to create a new todo
#[derive(Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

pub async fn create_todo(
    pool: web::Data<Pool>,
    data: web::Json<CreateTodoRequest>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get_conn().map_err(map_mysql_error)?;

    let query = "INSERT INTO todos (title, description, completed) VALUES (:title, :description, :completed)";
    conn.exec_drop(query, params! {
        "title" => &data.title,
        "description" => &data.description,
        "completed" => data.completed,
    }).map_err(map_mysql_error)?;

    Ok(HttpResponse::Created().json("Todo created successfully"))
}

// Function to update a todo
#[derive(Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

pub async fn edit_todo(
    pool: web::Data<Pool>,
    todo_id: web::Path<u64>,
    data: web::Json<UpdateTodoRequest>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get_conn().map_err(map_mysql_error)?;

    let mut query = "UPDATE todos SET".to_string();
    let mut params: Vec<mysql::Value> = vec![];

    // Check and append fields
    if let Some(title) = &data.title {
        query.push_str(" title = ?,");
        params.push(mysql::Value::from(title.clone())); // `Value::from()` converts a value to MySQL-compatible type
    }

    if let Some(description) = &data.description {
        query.push_str(" description = ?,");
        params.push(mysql::Value::from(description.clone()));
    }

    if let Some(completed) = data.completed {
        query.push_str(" completed = ?,");
        params.push(mysql::Value::from(completed));
    }

    // Remove the trailing comma if present
    if query.ends_with(',') {
        query.pop();
    }

    query.push_str(" WHERE id = ?");
    params.push(mysql::Value::from(todo_id.into_inner()));

    // Execute the query
    conn.exec_drop(query, params).map_err(map_mysql_error)?;

    Ok(HttpResponse::Ok().json("Todo updated successfully"))
}
// Function to delete a todo
pub async fn delete_todo(pool: web::Data<Pool>, todo_id: web::Path<u64>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get_conn().map_err(map_mysql_error)?;
    
    // Attempt to delete the todo item and handle errors properly
    let result: Option<u64> = conn
        .exec_first(
            "DELETE FROM todos WHERE id = ?",
            (todo_id.into_inner(),),
        )
        .map_err(map_mysql_error)?;

    // If the result is None, it means the todo with the given ID wasn't found
    match result {
        Some(_) => Ok(HttpResponse::Ok().json("Todo deleted successfully")),
        None => Ok(HttpResponse::NotFound().json("Todo not found")),
    }}
