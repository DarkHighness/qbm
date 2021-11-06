use crate::conf::{setup_config, Config};
use crate::env::collect_system_compiler_info;
use crate::execution::compile_file;
use env::collect_env_info;
use figment::Figment;
use rocket::futures::AsyncWriteExt;
use rocket::serde::json::Json;
use rocket::{Build, Rocket};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::path::Path;
use std::sync::atomic::AtomicPtr;
use std::sync::Arc;
use std::sync::Mutex;
use util::{BenchmarkInfo, BenchmarkTask, CompilerInfo, RemoteServerInfo};

mod conf;
mod env;
mod execution;
mod misc;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

#[get("/")]
fn index() -> &'static str {
    "QBM v0.1.0"
}

#[get("/env")]
fn env_info() -> Json<RemoteServerInfo> {
    Json(collect_env_info())
}

#[get("/compiler")]
fn compiler_info() -> Json<Vec<CompilerInfo>> {
    Json(collect_system_compiler_info())
}

#[post("/run", data = "<task>")]
fn run(task: Json<BenchmarkTask<'_>>) -> Json<BenchmarkInfo> {
    let task = task.0.borrow();
    let file_name = &task.task_uid[..8];
    let file_ext = Path::new(task.source_file)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    let no_match_ret = BenchmarkInfo {
        task_uid: task.task_uid.to_string(),
        message: "no compiler matches".to_string(),
        output: "".to_string(),
    };

    let compiler = setup_config()
        .extract::<Config>()
        .unwrap()
        .compilers
        .into_iter()
        .find(|p| p.name == task.compiler && p.version == task.compiler_version);

    if compiler.is_none() {
        Json(no_match_ret)
    } else {
        let compiler = compiler.unwrap();
        let compile_result = compile_file(
            &compiler.path,
            task.compiler_args,
            task.source_code,
            file_name,
            file_ext,
        );

        if compile_result.is_err() {
            Json(BenchmarkInfo {
                task_uid: task.task_uid.to_string(),
                message: compile_result.err().unwrap(),
                output: "".to_string(),
            })
        } else {
            Json(BenchmarkInfo {
                task_uid: task.task_uid.to_string(),
                message: compile_result.ok().unwrap(),
                output: "".to_string(),
            })
        }
    }
}

#[rocket::main]
async fn main() {
    let config = setup_config();

    rocket::custom(config)
        .mount("/", routes![index])
        .mount("/", routes![env_info])
        .mount("/", routes![compiler_info])
        .mount("/", routes![run])
        .launch()
        .await;
}
