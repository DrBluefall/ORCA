// Copyright (C) 2022, 2023 Alexander Bisono
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

#[allow(unused_imports)]
use tracing::{debug, error, info, trace, warn};

mod config;
mod logging;

use config::Config;

struct Data {}
type Error = anyhow::Error;
type Context<'ctx> = poise::Context<'ctx, Data, Error>;

#[tokio::main]
async fn main() {
    // right now, this'll *always* read from `orcaconf.toml`, but in the future
    // I may wanna add the ability to read a different config from, like, an
    // environment variable. Or a command line flag.
    let cfg = Config::read(None);

    let _log = logging::init(&cfg);

    info!(
        version = env!("CARGO_PKG_VERSION"),
        "initializing the Omniscient Recording Computer of Alterna"
    );
}
