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

use crate::config::Config;
use tracing::{level_filters::LevelFilter, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt};

fn log_formatter<S>() -> Layer<S> {
    Layer::default().with_target(false).with_level(true)
}

pub fn init(cfg: &Config) -> (WorkerGuard,) {
    let stdout_level = match cfg.log.level {
        0 => Level::TRACE,
        1 => Level::DEBUG,
        2 => Level::INFO,
        3 => Level::WARN,
        4.. => Level::ERROR,
    };

    let file_appender = tracing_appender::rolling::daily(&cfg.log.dir, "orca.log");
    let (nb_appender, guard) = tracing_appender::non_blocking(file_appender);

    let registry = tracing_subscriber::registry()
        .with(log_formatter().pretty().with_writer(std::io::stdout))
        .with(LevelFilter::from_level(stdout_level))
        .with(log_formatter().compact().with_ansi(false).with_writer(nb_appender));

    tracing::subscriber::set_global_default(registry).expect("Unable to initialize tracing");

    (guard,)
}
