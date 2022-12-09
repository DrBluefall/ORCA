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
use orca_config::Config;
use tracing::{level_filters::LevelFilter, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, Layer};

fn log_formatter<S>() -> tracing_subscriber::fmt::Layer<S> {
    tracing_subscriber::fmt::Layer::default()
        .with_target(false)
        .with_level(true)
}

pub fn init(cfg: &Config) -> (WorkerGuard,) {
    let stdout_level = match cfg.log.level {
        0 => Level::TRACE,
        1 => Level::DEBUG,
        2 => Level::INFO,
        3 => Level::WARN,
        4.. => Level::ERROR,
    };

    #[cfg(not(any(feature = "bot", feature = "server")))]
    compile_error!("One of the features `bot` or `server` must be enabled to use this crate");

    #[cfg(feature = "bot")]
    #[allow(unused_variables)]
    // Because it's annoying when, for some godforsaken reason, Rust Analyzer
    // misbehaves and applies both features.
    let logdir = &cfg.log.botfiles;

    #[cfg(feature = "server")]
    let logdir = &cfg.log.serverfiles;

    let file_appender = tracing_appender::rolling::daily(logdir, "orca.log");
    let (nb_appender, guard) = tracing_appender::non_blocking(file_appender);

    let registry = tracing_subscriber::registry()
        .with(
            log_formatter()
                .compact()
                .with_ansi(false)
                .with_writer(nb_appender),
        )
        .with(
            log_formatter()
                .pretty()
                .with_writer(std::io::stdout)
                .with_filter(LevelFilter::from_level(stdout_level)),
        );

    tracing_log::LogTracer::init().unwrap();

    tracing::subscriber::set_global_default(registry).expect("Unable to initialize tracing");

    (guard,)
}
