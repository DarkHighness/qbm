use rocket::{Build, Rocket};
use env::collect_env_info;
use rocket::serde::json::Json;
use util::{CompilerInfo, RemoteServerInfo};
use crate::conf::{Config, setup_config};
use crate::env::collect_system_compiler_info;

mod env;
mod compiler;
mod conf;
mod misc;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "QBM v0.1.0"
}

#[get("/env")]
fn env_info() -> Json<RemoteServerInfo> {
    Json ( collect_env_info() )
}

#[get("/compiler")]
fn compiler_info() -> Json<Vec<CompilerInfo>> { Json ( collect_system_compiler_info() )}

#[rocket::main]
async fn main() {
    let config = setup_config();

    let app_config = config.extract::<Config>().unwrap();

    rocket::custom(config)
        .mount("/", routes![index])
        .mount("/", routes![env_info])
        .mount("/", routes![compiler_info])
        .launch()
        .await;
}

