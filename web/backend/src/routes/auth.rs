use actix_web::{Responder, HttpResponse, web};
use serde::Deserialize;
use tracing::debug;

#[derive(Debug, Deserialize)]
pub struct SignInDetails {
    email: String,
    password: String,
}

#[actix_web::post("/signin")]
#[tracing::instrument]
pub async fn signin(details: web::Json<SignInDetails>) -> impl Responder {
    debug!("{:?}", details);
    HttpResponse::NotImplemented().finish()
}

#[actix_web::post("/signup")]
pub async fn signup() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
