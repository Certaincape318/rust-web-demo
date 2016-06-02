use config::reader;
use std::path::Path;
use config;
pub struct Config {
    config: config::types::Config,
}
impl Config {
    pub fn default() -> Self {
        Self::new("./web-root/config/web.conf")
    }
    pub fn new(path: &str) -> Self {
        match reader::from_file(Path::new(path)) {
            Ok(config) => Config { config: config },
            Err(err) => panic!("error creating config, the error is:{}", err),
        }
    }
    pub fn get_str(&self, key: &str) -> &str {
        match self.config.lookup_str(key) {
            Some(value) => value,
            _ => panic!("there is no string value in config for key:{}", key),
        }
    }
    pub fn get_i32(&self, key: &str) -> i32 {
        match self.config.lookup_integer32(key) {
            Some(value) => value,
            _ => panic!("there is no i32 value in config for key:{}", key),
        }
    }
}
