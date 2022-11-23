use std::{error::Error, ffi::OsStr, path::Path};

use serde::Deserialize;
use serde_json;
use toml;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Database {
    adapter: String,
    db_name: String,
    pool: u32,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Development {
    address: String,
    port: String,
    workers: u32,
    database: Database,
}
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Config {
    development: Development,
}

pub fn read_config(filepath: &Path) -> Result<Config, Box<dyn Error>> {
    println!("The file path is {:?}", filepath);
    let data = std::fs::read_to_string(&filepath)?;
    println!("toml data is {:#?}", data);
    match filepath.extension().and_then(OsStr::to_str) {
        Some("json") => {
            let content = serde_json::from_str(&data)?;
            Ok(content)
        }
        Some("toml") => {
            let content = toml::from_str(&data)?;
            Ok(content)
        }
        _ => Err(Box::from(
            "Json and toml config type have not been spported",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test.json");

        let content: Config = read_config(&path).unwrap();
        assert_eq!(content.development.address, "localhost");
        assert_eq!(content.development.port, "8000");
        assert_eq!(content.development.workers, 4);
        assert_eq!(content.development.database.adapter, "postgresql");
        assert_eq!(content.development.database.db_name, "blog_development");
        assert_eq!(content.development.database.pool, 5);
    }

    #[test]
    fn test_toml() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test.toml");

        let content = read_config(&path).unwrap();
        assert_eq!(content.development.address, "localhost");
        assert_eq!(content.development.port, "8000");
        assert_eq!(content.development.workers, 4);
        assert_eq!(content.development.database.adapter, "postgresql");
        assert_eq!(content.development.database.db_name, "blog_development");
        assert_eq!(content.development.database.pool, 5);
    }

    #[test]
    fn test_toml2() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("test2.toml");

        let content = read_config(&path).unwrap();
        assert_eq!(content.development.address, "localhost");
        assert_eq!(content.development.port, "8000");
        assert_eq!(content.development.workers, 10);
        assert_eq!(content.development.database.adapter, "post");
        assert_eq!(content.development.database.db_name, "dev_development");
        assert_eq!(content.development.database.pool, 5);
    }
}
