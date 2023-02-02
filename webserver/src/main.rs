use actix_session::{
    config::CookieContentSecurity, storage::CookieSessionStore, Session, SessionMiddleware,
};
use entity::sea_orm::Database;
use entity::sea_orm::ConnectionTrait;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use tracing::{info, log::LevelFilter};

use actix_files::Files;
use actix_web::{
    cookie::{Key, Cookie},
    http::header::LOCATION,
    web, App, HttpResponse, HttpServer, Responder,
};
use orca_config::Config;

mod api;

const DISCORD_API_BASE_URL: &'static str = "https://discord.com/api/v10";

#[actix_web::get("/login")]
async fn login(oauth: web::Data<BasicClient>, session: Session) -> impl Responder {
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_state) = oauth
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".into()))
        .set_pkce_challenge(challenge)
        .url();

    session
        .insert("pkce_verifier", verifier)
        .expect("Unable to insert verifier into session");

    session
        .insert("csrf_state", csrf_state)
        .expect("Unable to insert CSRF State into session");

    HttpResponse::Found()
        .append_header((LOCATION, auth_url.to_string()))
        .finish()
}

#[derive(serde::Deserialize)]
struct AuthRequest {
    pub code: String,
    pub state: String,
}

#[actix_web::get("/auth")]
async fn auth(
    session: Session,
    oauth: web::Data<BasicClient>,
    params: web::Query<AuthRequest>,
) -> HttpResponse {
    let verifier: PkceCodeVerifier = if let Some(x) = session.get("pkce_verifier").unwrap() {
        x
    } else {
        return HttpResponse::BadRequest().finish();
    };

    let code = AuthorizationCode::new(params.code.clone());
    let state: CsrfToken = if let Some(x) = session.get("csrf_state").unwrap() {
        x
    } else {
        return HttpResponse::BadRequest().finish();
    };

    if state.secret() != &params.state {
        return HttpResponse::BadRequest().finish();
    }

    let token = oauth
        .exchange_code(code)
        .set_pkce_verifier(verifier)
        .request_async(async_http_client)
        .await
        .expect("exchange_code failed");

    let client = reqwest::Client::new();

    let discord_user = match client
        .get(format!("{}/users/@me", DISCORD_API_BASE_URL))
        .bearer_auth(token.access_token().secret())
        .send()
        .await
    {
        Ok(u) => u.text().await.unwrap(),
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Found()
        .append_header((LOCATION, "/"))
        .cookie(Cookie::build("discord_user", &discord_user).secure(false).finish())
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // right now, this'll *always* read from `orcaconf.toml`, but in the future
    // I may wanna add the ability to read a different config from, like, an
    // environment variable. Or a command line flag.
    let cfg = Config::read(None);

    let _log = orca_logging::init(&cfg);

    let cookie_key = Key::generate();

    info!(
        version = env!("CARGO_PKG_VERSION"),
        "initializing the Omniscient Recording Computer of Alterna"
    );

    let client = oauth2::basic::BasicClient::new(
        ClientId::new(cfg.webserver.oauth.client_id),
        cfg.webserver.oauth.client_secret.map(ClientSecret::new),
        AuthUrl::new(cfg.webserver.oauth.auth_url).unwrap(),
        cfg.webserver
            .oauth
            .token_url
            .map(TokenUrl::new)
            .map(Result::unwrap),
    )
    .set_redirect_uri(RedirectUrl::new(cfg.webserver.oauth.redirect_url).unwrap());

    let mut dbopts = entity::sea_orm::ConnectOptions::new(cfg.database.url.clone());

    if let Some(log_queries) = &cfg.database.log_queries {
        let loglevel = match log_queries {
            0 => LevelFilter::Trace,
            1 => LevelFilter::Debug,
            2 => LevelFilter::Info,
            3 => LevelFilter::Warn,
            4.. => LevelFilter::Error,
        };
        dbopts.sqlx_logging(true).sqlx_logging_level(loglevel);
    } else {
        dbopts.sqlx_logging(false);
    }

    let db = Database::connect(dbopts)
        .await
        .expect("Failed to connect to database");

    let db = web::Data::new(db);

    info!(
        r#type = ?db.get_database_backend(),
        "Database connected & set up!",
    );

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), cookie_key.clone())
                    .cookie_content_security(CookieContentSecurity::Signed)
                    .cookie_secure(false)
                    .build(),
            )
            .service(login)
            .service(auth)
            .configure(api::configure)
            .service(
                Files::new(
                    "/",
                    cfg.assets
                        .webserver
                        .static_files
                        .as_ref()
                        .map(|x| x.as_path())
                        .unwrap_or_else(|| {
                            std::path::Path::new(concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/frontend/dist"
                            ))
                        }),
                )
                .index_file("index.html"),
            )
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(Config::read(None)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
