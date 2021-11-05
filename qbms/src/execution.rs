use std::fs;
use std::ops::Not;
use std::path::Path;
use std::process::Command;
use path_absolutize::*;

pub fn compile_file(
    compiler_path: &str,
    compiler_args: &str,
    source_code: &str,
    file_name: &str,
    file_ext: &str,
) -> Result<String, String> {
    let cache_dir = Path::new("./cache/");

    if cache_dir.exists().not() {
        fs::create_dir(cache_dir);
    }

    let source_file_name = format!("{}.{}", file_name, file_ext);
    let source_file_path = cache_dir
        .to_path_buf()
        .join(&source_file_name);

    fs::write(&source_file_path, source_code);

    let output = if cfg!(target_os = "windows") {
        Command::new(compiler_path)
            .current_dir(cache_dir.absolutize().unwrap())
            .args([
                compiler_args,
                &source_file_name,
                "-o",
                &format!("{}.exe", source_file_name),
            ])
            .output()
    } else {
        Command::new(compiler_path)
            .current_dir(cache_dir.absolutize().unwrap())
            .args([compiler_args, &source_file_name, "-o", file_name])
            .output()
    };

    if output.is_err() {
        Err(output.err().unwrap().to_string())
    } else {
        let output = output.unwrap();

        if output.status.success().not() {
            Err(String::from_utf8(output.stderr).unwrap())
        } else {
            Ok(String::from_utf8(output.stdout).unwrap())
        }
    }
}
