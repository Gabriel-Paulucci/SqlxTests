use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::sync::OnceCell;

use crate::config::Config;

static DATABASE: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn init_db(config: &Config) {
    DATABASE
        .get_or_init(|| async {
            PgPoolOptions::new()
                .connect(&config.database_url)
                .await
                .expect("Fail connect database")
        })
        .await;
}

pub fn get_db() -> &'static Pool<Postgres> {
    DATABASE.get().unwrap()
}
