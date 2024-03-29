use anyhow::Result;
use serde::Deserialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

pub mod data;
pub mod device;
pub mod system;

pub struct Db {
    db: Surreal<Client>,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

impl Db {
    pub async fn new(address: String, namespace: String, db_name: String) -> Result<Db> {
        let db = Surreal::new::<Ws>(address).await?;

        db.use_ns(namespace).use_db(db_name).await?;

        Ok(Db { db })
    }

    pub fn get_db(&self) -> &Surreal<Client> {
        &self.db
    }
}
