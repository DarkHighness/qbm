use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessorInfo {
    pub cpu_name: String,
    pub cpu_freq: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RemoteServerInfo {
    pub platform: String,
    pub kernel_version: String,
    pub os_version: String,
    pub total_memory: String,
    pub total_swap: String,
    pub cpu_cores: u64,
    pub cpu_logical_cores: u64,
    pub cpu_vendor: String,
    pub cpu_brand: String,
    pub cpus: Vec<ProcessorInfo>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CompilerInfo {
    pub name: String,
    pub version: String,
    pub target: Option<String>,
    pub thread_model: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BenchmarkInfo {
    pub task_uid: String,
    pub message: String,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
pub struct BenchmarkTask<'r> {
    pub task_uid: &'r str,
    pub compiler: &'r str,
    pub compiler_version: &'r str,
    pub source_file: &'r str,
    pub source_code: &'r str,
    pub compiler_args: &'r str,
    pub assembly: bool,
}
