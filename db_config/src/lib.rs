use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::fs;
use std::path::Path;
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    development: Development,
    database: Database,
}

#[derive(Debug, Serialize, Deserialize)]
struct Development {
    address: String,
    port: String,
    workers: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    adapter: String,
    db_name: String,
    pool: u32,
}

pub fn read_config(filepath: &Path) -> Result<Config, Box<dyn Error>> {
    let data = fs::read_to_string(filepath).expect("Please give a json or toml configuration file");
    if filepath.ends_with(".json") {
        let parsed: Config = serde_json::from_str(&data).unwrap();
        Some(parsed)
    } else if filepath.ends_with(".toml") {
        let parsed: Config = toml::from_str(&data).unwrap();
        Some(parsed)
    } else {
        panic!("This format config file have not been supported yet");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_check() {
        let filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("test.json");
        let parsed: Config = read_config(&filepath).unwrap();
        assert_eq!(parsed.development.address, "localhost");
        assert_eq!(parsed.development.port, "8080");
        assert_eq!(parsed.development.workers, 4);
        assert_eq!(parsed.database.adapter, "postgresql");
        assert_eq!(parsed.database.db_name, "blog_development");
        assert_eq!(parsed.database.pool, 5);
    }

    #[test]
    fn toml_check() {
        let filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test")
            .join("test.toml");
        let parsed: Config = read_config(&filepath).unwrap();
        assert_eq!(parsed.development.address, "localhost");
        assert_eq!(parsed.development.port, "8080");
        assert_eq!(parsed.development.workers, 4);
        assert_eq!(parsed.database.adapter, "postgresql");
        assert_eq!(parsed.database.db_name, "blog_development");
        assert_eq!(parsed.database.pool, 5);
    }
}
