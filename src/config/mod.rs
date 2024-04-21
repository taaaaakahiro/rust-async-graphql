use std::env;

pub struct Config {
    pub db: DBConfig,
}

pub struct DBConfig {
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        let db_config = DBConfig {
            port: env::var("PORT").expect("failed to load PORT"),
        };

        Self { db: db_config }
    }
}
