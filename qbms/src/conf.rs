use crate::env::collect_system_compiler_entry;
use crate::misc::QBM_DEFAULT_CONFIG_FILE_PATH;
use cached::proc_macro::cached;
use clap::{App, Arg};
use figment::providers::{Env, Format, Serialized, Toml};
use figment::value::{Dict, Value};
use figment::{map, Figment};
use serde::{Deserialize, Serialize};
use toml::value::Table;

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    port: Option<u16>,
    secrets: String,
    compilers: Option<Table>,
}

#[derive(Serialize, Deserialize)]
pub struct CompilerEntry {
    pub name: String,
    pub path: String,
    pub version: String,
}

impl From<CompilerEntry> for figment::value::Value {
    fn from(entry: CompilerEntry) -> Self {
        Value::from(map!["name" => entry.name, "path" => entry.path, "version" => entry.version ])
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub secrets: String,
    pub compilers: Vec<CompilerEntry>,
}

#[cached(size = 1, time = 15)]
pub fn setup_config() -> Figment {
    let matches = App::new("Quick Benchmark")
        .version("0.1.0")
        .author("Twiliness <https://github.com/DarkHighness>")
        .about("Run Benchmark with args")
        .arg(
            Arg::with_name("config")
                .short("f")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_file_path = if matches.value_of("config").is_none() {
        warn!(
            "Config path not set, use {} as default",
            QBM_DEFAULT_CONFIG_FILE_PATH
        );

        QBM_DEFAULT_CONFIG_FILE_PATH
    } else {
        matches.value_of("config").unwrap()
    };

    info!("Config path: {}", config_file_path);

    let map = map!["port" => Value::from(9000), "compilers" => Value::from(collect_system_compiler_entry())];

    Figment::new()
        .merge(Serialized::from(&map, "default"))
        .merge(Toml::file(config_file_path))
        .merge(Env::prefixed("QBMS_"))
        .join(rocket::Config::figment())
}
