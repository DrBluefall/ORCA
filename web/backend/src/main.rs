use actix_files::Files;
use actix_web::get;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use migration::Migrator;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::{info, log::LevelFilter};

use orca_config::Config;

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

    HttpServer::new(move || {
        App::new().service(checkhealth).service(Files::new(
            "/",
            config
                .assets
                .webserver
                .static_files
                .as_ref()
                .map(|x| x.as_str())
                .unwrap_or_else(|| "web/frontend/dist"),
        ).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
