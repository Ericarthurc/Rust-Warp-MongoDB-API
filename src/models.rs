use std::{fmt::Error, ptr::NonNull, str::FromStr};

use crate::database;
use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use mongodb::Cursor;
use serde::{Deserialize, Serialize};
use warp::Rejection;

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    pub product: String,
    pub condition: u8,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date: DateTime<Utc>,
}

impl Item {
    // pub async fn get_items(db: database::DB) -> Result<Vec<Item>> {
    //     let mut cursor = db.get_collection::<Item>("items").find(None, None).await?;

    //     let mut result: Vec<Item> = Vec::new();
    // }

    pub async fn get_item(id: String, db: database::DB) -> Result<Item, Rejection> {
        let oid = bson::oid::ObjectId::from_str(&id);
        let oid = match oid {
            Ok(oid) => oid,
            Err(_) => return Err(warp::reject::not_found()),
        };

        let find_results = db
            .get_collection::<Item>("items")
            .find_one(doc! {"_id": oid}, None)
            .await;

        match find_results {
            Ok(results) => match results {
                Some(item) => Ok(item),
                None => Err(warp::reject::not_found()),
            },
            Err(_error) => Err(warp::reject::not_found()),
        }
    }
}
