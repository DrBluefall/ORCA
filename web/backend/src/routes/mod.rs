use actix_web::web::{self, ServiceConfig};

mod auth;

pub fn configure(conf: &mut ServiceConfig) {
    conf.service(
        web::scope("/api").service(
            web::scope("/site")
                .service(auth::signin)
                .service(auth::signup)
                .service(auth::current_user),
        ),
    );
}
