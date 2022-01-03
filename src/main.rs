use config::Config;
use database::init_db;

pub mod config;
pub mod database;
pub mod models;

#[macro_use]
extern crate sqlx;

#[tokio::main]
async fn main() {
    let config: Config = Config::figment().extract().expect("Fail get config");

    init_db(&config).await;
}
