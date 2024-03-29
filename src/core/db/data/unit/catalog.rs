use crate::core::db::{Db, Record};
use crate::core::model::data::unit::catalog::UnitCatalog;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct UnitCatalogDb {
    id: String,
    name: String,
    description: String,
}

impl UnitCatalog {
    pub fn get_db_table_name() -> String {
        String::from("unit_catalog")
    }

    pub fn get_db_relate_name() -> String {
        String::from("device_unit_catalog")
    }

    pub async fn push(&self, db: &Db) -> Result<String> {
        let table_name = UnitCatalog::get_db_table_name();

        let _: Vec<Record> = db
            .get_db()
            .create(table_name.clone())
            .content(UnitCatalogDb {
                id: self.get_id().clone(),
                name: self.get_name().clone(),
                description: self.get_description().clone(),
            })
            .await?;

        let unit_catalog_table_id = format!("{}:⟨{}⟩", table_name, self.get_id().clone());

        for (_, unit) in self.get_units().iter() {
            let _ = unit.push(db).await?;
            unit.relate(db, unit_catalog_table_id.clone()).await?;
        }

        Ok(unit_catalog_table_id)
    }

    pub async fn relate(&self, db: &Db, id_to_relate: String) -> Result<()> {
        let table_name = UnitCatalog::get_db_table_name();
        let relate_table_name = UnitCatalog::get_db_relate_name();

        let sql = format!(
            "RELATE {} -> {} -> {}:⟨{}⟩",
            id_to_relate,
            relate_table_name,
            table_name,
            self.get_id()
        );

        db.get_db().query(sql).await?;

        Ok(())
    }
}
