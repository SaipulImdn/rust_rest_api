use actix_web::web;

use crate::controllers::{auth_controller, user_controller};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(auth_controller::login))
            .route("/register", web::post().to(auth_controller::register)),
    );
    cfg.service(
        web::scope("/user")
            .route("/edit", web::put().to(user_controller::edit_account))
            .route("/delete", web::delete().to(user_controller::delete_account)),
    );
}
