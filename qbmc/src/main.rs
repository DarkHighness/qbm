mod misc;

use crate::misc::QBM_DEFAULT_CONFIG_FILE_PATH;
use clap::{App, Arg, SubCommand};

#[macro_use]
extern crate util;

fn main() {
    util::init_logger();

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
        .subcommand(
            SubCommand::with_name("server")
                .about("Run qbm in server mode")
                .version("0.1.0")
                .author("Twiliness <https://github.com/DarkHighness>")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .help("Sets server port")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("secrets")
                        .short("s")
                        .long("secrets")
                        .help("Secrets to authorization")
                        .takes_value(true),
                ),
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

    info!("Config path: {}", config_file_path)
}
