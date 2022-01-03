use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn figment() -> Figment {
        Figment::new().merge(Toml::file("App.toml").nested())
    }
}
