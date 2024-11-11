use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::services::auth_service;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    login_data: web::Json<LoginRequest>,
) -> HttpResponse {
    match auth_service::login(&login_data.email, &login_data.password).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => HttpResponse::Unauthorized().body(err),
    }
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(
    register_data: web::Json<RegisterRequest>,
) -> HttpResponse {
    match auth_service::register(
        &register_data.username,
        &register_data.email,
        &register_data.password,
    )
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
