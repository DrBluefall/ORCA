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
    pub webserver: WebserverConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
    #[serde(default)]
    pub assets: AssetConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct WebserverConfig {
    pub oauth: DiscordOauthConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct DiscordOauthConfig {
    pub client_id: String,
    pub client_secret: Option<String>,
    pub auth_url: String,
    pub token_url: Option<String>,
    pub redirect_url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct BotConfig {
    pub token: String,
    pub owners: HashSet<u64>,
}

#[derive(serde::Deserialize, Debug)]
pub struct LogConfig {
    pub level: u8,
    pub botfiles: std::path::PathBuf,
    pub serverfiles: std::path::PathBuf,
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub url: String,
    pub log_queries: Option<u8>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct AssetConfig {
    #[serde(default)]
    pub xbattle: XBattleIcons,
    #[serde(default)]
    pub webserver: WebserverAssetConfig,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct WebserverAssetConfig {
    pub static_files: Option<std::path::PathBuf>,
}

#[derive(serde::Deserialize, Debug)]
pub struct XBattleIcons {
    pub tentatek: String,
    pub takaroka: String,
    pub splat_zones: String,
    pub tower_control: String,
    pub rainmaker: String,
    pub clam_blitz: String,
}

impl Default for XBattleIcons {
    fn default() -> Self {
        Self {
            tentatek: "ICN_XDIV_TTEK".into(),
            takaroka: "ICN_XDIV_TAKA".into(),
            splat_zones: "ICN_GOLDBADGE_SZ".into(),
            tower_control: "ICN_GOLDBADGE_TC".into(),
            rainmaker: "ICN_GOLDBADGE_RM".into(),
            clam_blitz: "ICN_GOLDBADGE_CB".into(),
        }
    }
}

impl Config {
    const DEFAULT_PATH: &str = "orcaconf.toml";

    pub fn read(path: impl Into<Option<String>>) -> Config {
        let path = path.into().unwrap_or_else(|| Self::DEFAULT_PATH.into());

        let conf_str = std::fs::read_to_string(&path).expect("Unable to read config");

        toml::from_str(&conf_str).expect("Unable to parse config")
    }
}
