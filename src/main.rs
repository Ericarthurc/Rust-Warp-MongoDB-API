use database::DB;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use warp::Filter;

mod database;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().ok();

    // Initialize mongodb connection
    let db = DB::init().await?;

    let home = warp::path::end()
        .and(DB::with_db(db.clone()))
        .and_then(handlers::inserter);

    warp::serve(home)
        .run(([127, 0, 0, 1], env::var("PORT").unwrap().parse().unwrap()))
        .await;

    Ok(())
}
