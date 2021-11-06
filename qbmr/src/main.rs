use crate::conf::setup_config;
use crate::engine::Engine;

mod conf;
mod engine;

fn main() {
    let config = setup_config().expect("unable to load config.");

    let engine = Engine::new(&config.compilers, "./cache/", "./build").unwrap();

    let result = engine.execute(
        "clang++-13.0.0",
        "",
        "#include<cstdio>\nint main(){ printf(\"%d\\n\", 5);\n return 0; }",
        "",
        "main.cpp",
    );

    println!("{:?}", result)
}
