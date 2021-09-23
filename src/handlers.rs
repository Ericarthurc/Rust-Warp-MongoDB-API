use crate::database;
use crate::models;
use chrono::Utc;
use database::DB;
use warp::{Rejection, Reply};

pub async fn inserter(db: DB) -> Result<impl Reply, Rejection> {
    let items_coll = db.mdb.collection::<models::Item>("items");

    let test_input = models::Item {
        product: "Macbook Pro 2012".to_string(),
        condition: 100,
        date: Utc::now(),
    };

    let insert_results = items_coll.insert_one(&test_input, None).await.unwrap();
    Ok(insert_results.inserted_id.to_string())
}
