use actix_web::web;

use crate::auth::handler::{
    signin,
    signup
};

pub fn scoped_auth(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signin", web::post().to(signin))
            .route("/signup", web::post().to(signup))
    );
}