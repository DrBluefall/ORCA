use actix_files::Files;
use actix_session::storage::RedisActorSessionStore;
use actix_session::SessionMiddleware;
use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use cookie::Key;
use migration::Migrator;
use sea_orm::{ConnectOptions, ConnectionTrait, Database};
use sea_orm_migration::MigratorTrait;
use tracing::{info, log::LevelFilter};

use orca_config::Config;

mod routes;

#[get("/checkhealth")]
async fn checkhealth() -> impl Responder {
    HttpResponse::Ok().body("All is well!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // For now, this'll just read orcaconf.toml.
    //
    // TODO: add some kind of command line option to read a different config. Or
    // an environment variable.
    let config = Config::read(None);

    let _guard = orca_logging::init(&config);

    info!(
        version = env!("CARGO_PKG_VERSION"),
        "initializing the Omniscient Recording Computer of Alterna"
    );

    let mut dbopts = ConnectOptions::new(config.database.url.clone());

    if let Some(log_queries) = &config.database.log_queries {
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

    Migrator::up(&db, None)
        .await
        .expect("Failed to apply migrations");

    info!(
        r#type = ?db.get_database_backend(),
        "Database connected & set up!",
    );

    let db_dat = web::Data::new(db);

    let secret_key_path = std::path::Path::new(&config.webserver.secret_key_path);

    let secret_key: Key;

    if secret_key_path.exists() {
        // Read a key we already have saved...
        secret_key = Key::from(&std::fs::read(secret_key_path).unwrap());
    } else {
        // ...or generate a new secret key and save it to the configured path
        secret_key = Key::generate();
        std::fs::write(secret_key_path, secret_key.master()).unwrap();
    }

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::new(&config.webserver.redis_url),
                secret_key.clone(),
            ))
            .app_data(db_dat.clone())
            .service(checkhealth)
            .configure(routes::configure)
            .service(
                Files::new(
                    "/",
                    config
                        .assets
                        .webserver
                        .static_files
                        .as_ref()
                        .map(|x| x.as_str())
                        .unwrap_or_else(|| "web/frontend/dist"),
                )
                .index_file("index.html"),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
