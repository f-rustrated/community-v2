use std::sync::OnceLock;

#[derive(Debug)]
pub struct Config {
    pub(crate) postgresql_url: String,
    pub(crate) eventstoredb_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            postgresql_url: std::env::var("POSTGRESQL_URL").expect("POSTGRESQL_URL must be set"),
            eventstoredb_url: std::env::var("EVENTSTOREDB_URL").expect("EVENTSTOREDB_URL must be set"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        dotenv::dotenv().ok();
        Config::new()
    })
}