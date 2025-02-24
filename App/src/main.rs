mod db;

use sqlx::{MySql, Pool};
use dotenvy::dotenv;
use std::env;
use tokio;
fn main() -> Result<(), sqlx::Error> {
    let db_config = db::connect_to_db();
    println!("hejka");
    Ok(())
}
