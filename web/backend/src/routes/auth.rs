use actix_web::{Responder, HttpResponse};

#[actix_web::post("/signin")]
pub async fn signin() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
