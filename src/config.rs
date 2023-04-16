use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: Server,
    pub database: Database,
}

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub port: Option<u16>,
    pub worker_amount: Option<usize>,
}

#[derive(Deserialize)]
pub struct Database {
    pub postgres_url: String,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let content = std::fs::read_to_string(path)
            .expect("Failed to read configuration file");

        return toml::from_str::<Config>(content.as_str())
            .expect("Some required values are missing from your configuration file.");
    }
}
