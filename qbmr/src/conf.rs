use figment::providers::{Env, Format, Toml};
use figment::value::{Dict, Map};
use figment::{Error, Figment};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct CompilerConfig {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) version: String,
    pub(crate) command: String,
    pub(crate) extensions: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub(crate) compilers: Vec<CompilerConfig>,
}

pub fn setup_config() -> Result<Config, Error> {
    Figment::new()
        .merge(Toml::file("qbmr.toml"))
        .merge(Env::prefixed("QMBR_"))
        .extract()
}
