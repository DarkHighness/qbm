use std::ops::Not;
use std::process::Command;

use cached::proc_macro::cached;
use regex::Regex;
use sysinfo::{ProcessorExt, System, SystemExt};

use util::{CompilerInfo, ProcessorInfo, RemoteServerInfo};

use crate::conf::CompilerEntry;

#[cached(size = 1, time = 120)]
pub fn collect_env_info() -> RemoteServerInfo {
    let mut sys = System::new_all();

    sys.refresh_all();

    let cpu = sys.global_processor_info();

    RemoteServerInfo {
        platform: sys.name().unwrap_or_default(),
        kernel_version: sys.kernel_version().unwrap_or_default(),
        os_version: sys.long_os_version().unwrap_or_default(),
        total_memory: format!("{:.2} GB", (sys.total_memory() as f64) / 1024.0 / 1024.0),
        total_swap: format!("{:.2} GB", (sys.total_swap() as f64) / 1024.0 / 1024.0),
        cpu_vendor: cpu.vendor_id().to_string(),
        cpu_brand: cpu.brand().to_string(),
        cpu_cores: sys.physical_core_count().unwrap_or_default() as u64,
        cpu_logical_cores: sys.processors().len() as u64,
        cpus: sys
            .processors()
            .into_iter()
            .map(|c| ProcessorInfo {
                cpu_name: c.name().to_string(),
                cpu_freq: format!("{}GHz", (c.frequency() as f64) / 1000.0),
            })
            .collect(),
    }
}

pub fn collect_system_compiler_entry() -> Vec<CompilerEntry> {
    let clang = compiler_check_system_clang();
    let gcc = compiler_check_system_gcc();

    let mut entries = vec![];

    if clang.is_some() {
        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(["-Command", "&{ (gcm clang++).Source}"])
                .output()
        } else {
            Command::new("sh").arg("-c").arg("which clang++").output()
        };

        if output.is_ok() {
            let path = String::from_utf8(output.unwrap().stdout).unwrap();

            let info = clang.unwrap();

            entries.push(CompilerEntry {
                name: info.name,
                version: info.version,
                path,
            })
        }
    }

    if gcc.is_some() {
        let output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .args(["-Command", "& {(gcm g++).Source}"])
                .output()
        } else {
            Command::new("sh").arg("-c").arg("which g++").output()
        };

        if output.is_ok() {
            let path = String::from_utf8(output.unwrap().stdout).unwrap();

            let info = gcc.unwrap();

            entries.push(CompilerEntry {
                name: info.name,
                version: info.version,
                path,
            })
        }
    }

    entries
}

#[cached(size = 1, time = 120)]
pub fn collect_system_compiler_info() -> Vec<CompilerInfo> {
    return vec![compiler_check_system_clang(), compiler_check_system_gcc()]
        .into_iter()
        .filter(|e| e.is_some())
        .map(|e| e.unwrap())
        .collect();
}

fn compiler_check_system_clang() -> Option<CompilerInfo> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "clang++ --version"])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("clang++ --version")
            .output()
    };

    if output.is_err() {
        return None;
    }

    let output = output.unwrap();

    if output.status.success().not() {
        return None;
    }

    let clang_output = String::from_utf8(output.stdout).unwrap();
    let clang_version_regex = Regex::new(r"clang version (?P<version>\d+\.\d+\.\d+\w*)\n").unwrap();
    let clang_version = &clang_version_regex.captures(&clang_output).unwrap()["version"];
    let clang_target_regex = Regex::new(r"Target: (?P<target>[-\w]+)\n").unwrap();
    let clang_target = &clang_target_regex.captures(&clang_output).unwrap()["target"];
    let clang_thread_model_regex = Regex::new(r"Thread model: (?P<thread_model>\w+)\n").unwrap();
    let clang_thread_model =
        &clang_thread_model_regex.captures(&clang_output).unwrap()["thread_model"];

    Some(CompilerInfo {
        name: "clang++".to_string(),
        version: clang_version.to_string(),
        target: Some(clang_target.to_string()),
        thread_model: Some(clang_thread_model.to_string()),
    })
}

fn compiler_check_system_gcc() -> Option<CompilerInfo> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "g++ --version"]).output()
    } else {
        Command::new("sh").arg("-c").arg("g++ --version").output()
    };

    if output.is_err() {
        return None;
    }

    let output = output.unwrap();

    if output.status.success().not() {
        return None;
    }

    let gcc_output = String::from_utf8(output.stdout).unwrap();
    let gcc_regex = Regex::new(r"[\w\.\+]+ \((?P<target>.+)\) (?P<version>\d+\.\d+\.\d+)").unwrap();
    let gcc_version = &gcc_regex.captures(&gcc_output).unwrap()["version"];
    let gcc_target = &gcc_regex.captures(&gcc_output).unwrap()["target"];

    Some(CompilerInfo {
        name: "g++".to_string(),
        version: gcc_version.to_string(),
        target: Some(gcc_target.to_string()),
        thread_model: None,
    })
}
