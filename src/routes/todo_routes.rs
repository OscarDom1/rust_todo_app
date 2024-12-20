use actix_web::web;
use crate::handlers::todo::{get_todo, get_todos, edit_todo, delete_todo}; // Import the handler functions

pub fn todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("/", web::get().to(get_todos)) // Get all todos
            .route("/{id}", web::get().to(get_todo)) // Get a single todo by ID
            .route("/{id}", web::put().to(edit_todo)) // Edit a todo by ID
            .route("/{id}", web::delete().to(delete_todo)) // Delete a todo by ID
    );
}
