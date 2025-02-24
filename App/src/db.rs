use sqlx::{MySql, Pool};
use dotenvy::dotenv;
use std::env;
use tokio;

pub async fn connect_to_db() -> Result<(), sqlx::Error> {
    dotenv().ok(); //ładuje zmienne z pliku env
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = Pool::<MySql>::connect(&database_url).await?;

    Ok(())
}
