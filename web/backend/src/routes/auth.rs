use actix_session::Session;
use actix_web::{http::header::SET_COOKIE, web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use entity::orcasite_account::{self, Entity as OrcaAccount};
use migration::tests_cfg::json;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AuthInfo {
    email: String,
    password: String,
}

#[actix_web::post("/signin")]
#[tracing::instrument(skip(session))]
pub async fn signin(
    details: web::Json<AuthInfo>,
    db: web::Data<DatabaseConnection>,
    session: Session,
) -> impl Responder {
    let result = OrcaAccount::find()
        .filter(orcasite_account::Column::Email.eq::<&str>(&details.email))
        .one(&**db)
        .await
        .unwrap();

    if let Some(potential_match) = result {
        let ag2 = Argon2::default();

        let pw_hash = PasswordHash::new(&potential_match.passhash_str).unwrap();

        match ag2.verify_password(details.password.as_bytes(), &pw_hash) {
            Ok(()) => {
                // Password verified! Let's get them in.
                session.renew();
                session.insert("orca_user_id", potential_match.id).unwrap();
                HttpResponse::Ok().finish()
            }
            Err(e) => match e {
                argon2::password_hash::Error::Password => HttpResponse::Unauthorized().finish(),
                _ => {
                    if cfg!(debug_assertions) {
                        HttpResponse::InternalServerError()
                            .json(json!({ "err": format!("{:?}", e) }))
                    } else {
                        HttpResponse::InternalServerError().finish()
                    }
                }
            },
        }
    } else {
        return HttpResponse::NotFound().finish();
    }
}

#[derive(Debug, Deserialize)]
pub struct NewSignUp {
    email: String,
    username: String,
    password: String,
}

#[actix_web::post("/signup")]
#[tracing::instrument(skip(session))]
pub async fn signup(
    signup_info: web::Json<NewSignUp>,
    db: web::Data<DatabaseConnection>,
    session: Session,
) -> impl Responder {
    let dupe_email_check = OrcaAccount::find()
        .filter(orcasite_account::Column::Email.eq::<&str>(&signup_info.email))
        .one(&**db)
        .await
        .unwrap();

    if dupe_email_check.is_some() {
        // This user already exists.
        return HttpResponse::BadRequest().json(json!({ "already_exists": "email" }));
    }

    let dupe_uname_check = OrcaAccount::find()
        .filter(orcasite_account::Column::Username.eq::<&str>(&signup_info.username))
        .one(&**db)
        .await
        .unwrap();

    if dupe_uname_check.is_some() {
        // This user already exists.
        return HttpResponse::BadRequest().json(json!({ "already_exists": "uname" }));
    }

    match Argon2::default().hash_password(
        signup_info.password.as_bytes(),
        &SaltString::generate(&mut OsRng),
    ) {
        Ok(pw_hash) => {
            let uid = Uuid::now_v7();

            let new_user = orcasite_account::ActiveModel {
                email: ActiveValue::Set(signup_info.email.clone()),
                passhash_str: ActiveValue::Set(pw_hash.to_string()),
                username: ActiveValue::Set(signup_info.username.clone()),
                id: ActiveValue::Set(uid),
            };

            new_user.insert(&**db).await.unwrap();
            session.renew();
            session.insert("orca_user_id", uid).unwrap();

            HttpResponse::Created().finish()
        }
        Err(e) => {
            if cfg!(debug_assertions) {
                HttpResponse::InternalServerError().json(json!({ "err": format!("{:?}", e) }))
            } else {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}
