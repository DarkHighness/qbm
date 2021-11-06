use crate::conf::CompilerConfig;
use path_absolutize::Absolutize;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;

struct EngineCompiler {
    name: String,
    path: String,
    command: String,
    extensions: HashSet<String>,
}

pub struct Engine {
    compilers: Vec<EngineCompiler>,
    source_directory: String,
    build_directory: String,
}

impl Engine {
    pub fn new(
        compilers: &Vec<CompilerConfig>,
        source_directory: &str,
        build_directory: &str,
    ) -> Self {
        let source_directory_path = Path::new(source_directory);

        if !source_directory_path.exists() {
            fs::create_dir_all(source_directory_path).expect("unable to crate source directory.")
        }

        let build_directory_path = Path::new(build_directory);

        if !build_directory_path.exists() {
            fs::create_dir_all(build_directory_path).expect("unable to create build directory.")
        }

        let compilers = compilers
            .as_slice()
            .into_iter()
            .map(|c| EngineCompiler {
                name: format!("{}-{}", c.name, c.version),
                path: c.path.clone(),
                command: c.command.clone(),
                extensions: HashSet::from_iter(
                    c.extensions.as_slice().into_iter().map(|e| e.to_string()),
                ),
            })
            .collect();

        Engine {
            compilers,
            source_directory: source_directory.to_string(),
            build_directory: build_directory.to_string(),
        }
    }

    pub fn execute(
        &self,
        compiler: &str,
        compiler_args: &str,
        source_code: &str,
        execution_args: &str,
        file_name: &str,
    ) -> Result<String, String> {
        let (compiler, source_file_path, target_file_path) = self
            .setup_execution_env(compiler, source_code, file_name)
            .expect("unable to setup execution env");

        let compiler_result = Self::do_compile(compiler_args, compiler, source_file_path, &target_file_path);

        return if compiler_result.is_ok() {
            Self::execute_program(execution_args, target_file_path)
        } else {
            compiler_result
        };
    }

    fn execute_program(execution_args: &str, target_file_path: String) -> Result<String, String> {
        let execution_output = Command::new(
            Path::new(target_file_path.as_str())
                .absolutize()
                .unwrap()
                .to_str()
                .unwrap(),
        )
            .arg(execution_args)
            .output();

        if execution_output.is_err() {
            Err(execution_output.err().unwrap().to_string())
        } else {
            let execution_output = execution_output.unwrap();
            let execution_stderr = String::from_utf8(execution_output.stderr).unwrap();
            let execution_stdout = String::from_utf8(execution_output.stdout).unwrap();

            if !execution_output.status.success() {
                Err(format!("execute program failed, code: {}\n{}", execution_output.status, execution_stderr))
            } else {
                Ok(execution_stdout)
            }
        }
    }

    fn do_compile(compiler_args: &str, compiler: &EngineCompiler, source_file_path: String, target_file_path: &String) -> Result<String, String> {
        let compiler_command = compiler
            .command
            .replace("${COMPILER}", compiler.path.as_str())
            .replace("${COMPILER_ARGS}", compiler_args)
            .replace(
                "${SOURCE_FILES}",
                Path::new(source_file_path.as_str())
                    .absolutize()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            )
            .replace(
                "${TARGET_FILE_NAME}",
                Path::new(target_file_path.as_str())
                    .absolutize()
                    .unwrap()
                    .to_str()
                    .unwrap(),
            );

        let compiler_output = if cfg!(target_os = "windows") {
            Command::new("powershell")
                .arg("-Command")
                .arg(&format!("&{{{}}}", compiler_command.as_str()))
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&format!("&{{{}}}", compiler_command.as_str()))
                .output()
        };

        if compiler_output.is_err() {
            return Err(format!(
                "run command failed, {}",
                compiler_output.err().unwrap()
            ));
        }

        let compiler_output = compiler_output.unwrap();
        let compiler_stderr = String::from_utf8(compiler_output.stderr).unwrap();
        let compiler_stdout = String::from_utf8(compiler_output.stdout).unwrap();

        if !compiler_output.status.success() {
            return Err(format!(
                "compile failed, code {}\n{}",
                compiler_output.status, compiler_stderr
            ));
        }

        let compiler_result = if compiler_stdout.is_empty() && compiler_stderr.is_empty() {
            Ok("compile ok".to_string())
        } else if compiler_stderr.is_empty() {
            Ok(compiler_stdout)
        } else {
            Err(compiler_stderr)
        };
        compiler_result
    }

    fn setup_execution_env(
        &self,
        compiler: &str,
        source_code: &str,
        file_name: &str,
    ) -> Result<(&EngineCompiler, String, String), String> {
        let file_extension = Path::new(file_name)
            .extension()
            .expect("invalid file name")
            .to_str()
            .unwrap();

        let compiler_option = self
            .compilers
            .as_slice()
            .into_iter()
            .find(|c| c.name.as_str() == compiler && c.extensions.contains(file_extension));

        if compiler_option.is_none() {
            return Err("unable to find specific compiler".to_string());
        }

        let compiler = compiler_option.unwrap();
        let file_base_name = Path::new(file_name)
            .file_stem()
            .expect("invalid file name")
            .to_str()
            .unwrap();

        let source_file_path_buf = Path::new(self.source_directory.as_str()).join(file_name);
        let source_file_path = source_file_path_buf.as_path();

        fs::write(source_file_path, source_code).expect("unable to write source file");

        let target_file_path_buf = Path::new(self.build_directory.as_str()).join(file_base_name);
        let target_file_path = target_file_path_buf.as_path();

        Ok((
            compiler,
            source_file_path.to_str().unwrap().to_string(),
            target_file_path.to_str().unwrap().to_string(),
        ))
    }
}
