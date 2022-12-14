// Copyright (C) 2022 Alexander Bisono
//
// This file is part of O.R.C.A., a Splatoon 3 Discord Bot.
//
// O.R.C.A is free software: you can redistribute it and/or modify it under the
// terms of the GNU Affero General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// O.R.C.A is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU Affero General Public License for more
// details.
//
// You should have received a copy of the GNU Affero General Public License
// along with O.R.C.A. If not, see <https://www.gnu.org/licenses/>.

use migration::Migrator;
#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn, Level};

mod commands;

use orca_config::Config;
use poise::serenity_prelude as serenity;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::prelude::*;
use sysinfo::SystemExt;
use tracing::log::LevelFilter;

pub struct Data {
    pub stats: Statistics,
    pub db: DatabaseConnection,
    pub conf: Config,
}

/// Statistics about the bot's operations.
pub struct Statistics {
    pub start_time: std::time::Instant,
    pub sysinfo: tokio::sync::RwLock<sysinfo::System>,
}

pub type Error = anyhow::Error;
pub type Context<'ctx> = poise::Context<'ctx, Data, Error>;

#[tokio::main]
async fn main() {
    // right now, this'll *always* read from `orcaconf.toml`, but in the future
    // I may wanna add the ability to read a different config from, like, an
    // environment variable. Or a command line flag.
    let cfg = Config::read(None);

    let _log = orca_logging::init(&cfg);

    info!(
        version = env!("CARGO_PKG_VERSION"),
        "initializing the Omniscient Recording Computer of Alterna"
    );

    let mut dbopts = ConnectOptions::new(cfg.database.url.clone());

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

    Migrator::up(&db, None)
        .await
        .expect("Failed to apply migrations");

    info!(
        r#type = ?db.get_database_backend(),
        "Database connected & set up!",
    );

    let fw = poise::Framework::builder()
        .token(&cfg.bot.token)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::register_commands(),
                commands::system::system(),
                commands::system::system_info(),
                commands::dossier::dossier(),
                commands::dossier::dossier_get(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                mention_as_prefix: true,
                ..Default::default()
            },
            owners: cfg
                .bot
                .owners
                .iter()
                .map(|x| serenity::UserId::from(*x))
                .collect::<std::collections::HashSet<_>>(),
            ..Default::default()
        })
        .setup(|_, _, _| {
            Box::pin(async move {
                Ok(Data {
                    conf: cfg,
                    db,
                    stats: Statistics {
                        start_time: std::time::Instant::now(),
                        sysinfo: tokio::sync::RwLock::new(sysinfo::System::new_all()),
                    },
                })
            })
        });

    fw.run_autosharded()
        .await
        .expect("Fatal error in bot runtime");
}
