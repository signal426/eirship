use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db_url: String,
    pub log_prefix: String,
}
