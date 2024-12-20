use actix_web::{web, App, HttpServer};
use crate::config::database::get_db_pool;
use crate::routes::{auth_routes, todo_routes}; 

mod config;
mod handlers;
mod models;
mod routes;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();  // Load environment variables from the .env file
    env_logger::init();     // Initialize logger

    // Initialize the MySQL database pool
    let pool = get_db_pool().await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // Share database connection pool
            .wrap(actix_web::middleware::Logger::default())  // Log all incoming requests

            // Register routes
    .configure(auth_routes::auth_routes)  // Authentication routes
            .configure(todo_routes::todo_routes)  // Todo routes
    })
    .bind("127.0.0.1:7000")?
    .run()
    .await
}
