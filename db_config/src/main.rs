use db_config::read_config;
use std::path::Path;

fn main() {
    let filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test.toml");
    let content = read_config(&filepath).unwrap();

    println!("{:#?}", content);
}
