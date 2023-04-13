use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub server: String,
}
impl Config {
    pub fn get_end_point(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
impl Config {
    pub fn load_config() -> Self {
        println!("Using Default config");
        Config {
            host: "127.0.0.1".into(),
            port: "5555".into(),
            server: "test".into(),
        }
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
