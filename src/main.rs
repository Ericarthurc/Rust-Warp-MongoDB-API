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

    // let home = warp::path::end()
    //     .and(DB::with_db(db.clone()))
    //     .and_then(handlers::inserter);

    let api_route = warp::path("api");
    let items_route = api_route.and(warp::path("items"));
    let items_route_id = items_route.and(warp::path::param::<String>());

    let gets = warp::get()
        .and(items_route)
        .and(DB::with_db(db.clone()))
        .and_then(handlers::get_items_handler);

    // let route = warp::path::param().and_then(|id: u32| handlers::get_item);
    let get = warp::get()
        .and(items_route_id)
        .and(DB::with_db(db.clone()))
        .and_then(handlers::get_item_handler);

    // let routes = gets.or(get);

    warp::serve(get)
        .run(([127, 0, 0, 1], env::var("PORT").unwrap().parse().unwrap()))
        .await;

    Ok(())
}
