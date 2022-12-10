use tracing::info;

use actix_files::Files;
use actix_web::{App, HttpServer};
use orca_config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // right now, this'll *always* read from `orcaconf.toml`, but in the future
    // I may wanna add the ability to read a different config from, like, an
    // environment variable. Or a command line flag.
    let cfg = Config::read(None);

    let _log = orca_logging::init(&cfg);

    info!(
        version = env!("CARGO_PKG_VERSION"),
        "initializing the Omniscient Recording Computer of Alterna"
    );

    HttpServer::new(move || {
        App::new().service(
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
                            "/frontend/public"
                        ))
                    }),
            )
            .index_file("index.html"),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
