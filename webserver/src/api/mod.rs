//! Endpoints for reading & manipulating data.

use actix_web::web;
use actix_web::web::ServiceConfig;

pub mod profile;

#[doc(hidden)]
pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api").service(profile::get_profile));
}
