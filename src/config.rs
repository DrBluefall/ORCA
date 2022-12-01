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
use std::collections::HashSet;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub bot: BotConfig,
    pub log: LogConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct BotConfig {
    pub token: String,
    pub owners: HashSet<u64>,
}

#[derive(serde::Deserialize, Debug)]
pub struct LogConfig {
    pub level: u8,
    pub dir: std::path::PathBuf,
}

impl Config {
    const DEFAULT_PATH: &str = "orcaconf.toml";

    pub fn read(path: impl Into<Option<String>>) -> Config {
        let path = path.into().unwrap_or_else(|| Self::DEFAULT_PATH.into());

        let conf_str = std::fs::read_to_string(&path).expect("Unable to read config");

        toml::from_str(&conf_str).expect("Unable to parse config")
    }
}
