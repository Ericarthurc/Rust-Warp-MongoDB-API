use crate::database;
use crate::models;
use chrono::Utc;
use database::DB;
use warp::reply::json;
use warp::{Rejection, Reply};

// pub async fn inserter(db: DB) -> Result<impl Reply, Rejection> {
//     let items_coll = db.mdb.collection::<models::Item>("items");

//     let test_input = models::Item {
//         product: "Macbook Pro 2012".to_string(),
//         condition: 100,
//         date: Utc::now(),
//     };

//     let insert_results = items_coll.insert_one(&test_input, None).await.unwrap();
//     Ok(insert_results.inserted_id.to_string())
// }

pub async fn get_items_handler(db: DB) -> Result<impl Reply, Rejection> {
    let items_coll = db.mdb.collection::<models::Item>("items");

    let items = items_coll.find(None, None).await;
    match items {
        Ok(state) => println!("{:?}", state),
        Err(error) => println!("{:?}", error),
    }
    Ok("okay")
}

pub async fn get_item_handler(id: String, db: DB) -> Result<impl Reply, Rejection> {
    let item = models::Item::get_item(id, db).await.unwrap();
    Ok(json(&item))
}

// pub async fn create_item() -> Result<impl Reply, Rejection> {}
// pub async fn update_item() -> Result<impl Reply, Rejection> {}
// pub async fn delete_item() -> Result<impl Reply, Rejection> {}
