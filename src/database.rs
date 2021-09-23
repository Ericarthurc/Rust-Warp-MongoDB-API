use mongodb::{options::ClientOptions, options::ResolverConfig, Client, Database};
use std::env;
use std::{convert::Infallible, error::Error};
use warp::Filter;

#[derive(Clone, Debug)]
pub struct DB {
    pub mdb: Database,
}

impl DB {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let options = ClientOptions::parse_with_resolver_config(
            env::var("MONGO_URI").unwrap(),
            ResolverConfig::cloudflare(),
        )
        .await?;

        let client = Client::with_options(options)?;

        Ok(Self {
            mdb: client.database(env::var("MONGO_DB").unwrap().as_str()),
        })
    }

    pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}
