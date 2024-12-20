use crate::handlers::auth::{register_user, validate_user}; 
use crate::utils::jwt::generate_jwt;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use mysql::Pool;

// Register request structure
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// Login request structure
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Route handler for user registration
pub async fn register(data: web::Json<RegisterRequest>, pool: web::Data<Pool>) -> impl Responder {
    match register_user(&pool, &data.name, &data.email, &data.password).await {
        Ok(_) => HttpResponse::Created().json("User registered successfully"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to register user"),
    }
}


// Route handler for user login
pub async fn login(data: web::Json<LoginRequest>, pool: web::Data<Pool>) -> impl Responder {
    match validate_user(&pool, &data.email, &data.password).await {
        Ok(valid_user) => {
            if valid_user {
                match generate_jwt(&data.email) {
                    Ok(token) => HttpResponse::Ok().json(token),
                    Err(_) => HttpResponse::InternalServerError().json("Error generating token"),
                }
            } else {
                HttpResponse::Unauthorized().json("Invalid email or password")
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Error validating user"),
    }
}

// Function to register the routes
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register)))
       .service(web::resource("/login").route(web::post().to(login)));
}
