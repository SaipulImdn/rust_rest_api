use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;

use crate::services::user_service;

#[derive(Deserialize)]
pub struct EditAccountRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

pub async fn edit_account(
    user_id: web::Path<Uuid>,
    edit_data: web::Json<EditAccountRequest>,
) -> HttpResponse {
    match user_service::edit_account(*user_id, edit_data.0).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}

#[derive(Deserialize)]
pub struct DeleteAccountRequest {
    pub password: String,
}

pub async fn delete_account(
    user_id: web::Path<Uuid>,
    delete_data: web::Json<DeleteAccountRequest>,
) -> HttpResponse {
    match user_service::delete_account(*user_id, &delete_data.password).await {
        Ok(_) => HttpResponse::Ok().body("Account deleted successfully"),
        Err(err) => HttpResponse::BadRequest().body(err),
    }
}
