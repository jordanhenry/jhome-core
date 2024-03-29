use crate::core::db::{Db, Record};
use crate::core::model::data::measurement::catalog::MeasurementCatalog;
use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct MeasurementCatalogDb {
    id: String,
    name: String,
    description: String,
}

impl MeasurementCatalog {
    pub fn get_db_table_name() -> String {
        String::from("measurement_catalog")
    }

    pub fn get_db_relate_name() -> String {
        String::from("device_measurement_catalog")
    }

    pub async fn push(&self, db: &Db) -> Result<String> {
        let table_name = MeasurementCatalog::get_db_table_name();

        let _: Vec<Record> = db
            .get_db()
            .create(table_name.clone())
            .content(MeasurementCatalogDb {
                id: self.get_id().clone(),
                name: self.get_name().clone(),
                description: self.get_description().clone(),
            })
            .await?;

        let measurement_catalog_table_id = format!("{}:⟨{}⟩", table_name, self.get_id().clone());

        for (_, measurement_definition) in self.get_measurement_definitions().iter() {
            let _ = measurement_definition.push(db).await?;
            measurement_definition
                .relate(db, measurement_catalog_table_id.clone())
                .await?;
        }

        Ok(measurement_catalog_table_id)
    }

    pub async fn relate(&self, db: &Db, id_to_relate: String) -> Result<()> {
        let table_name = MeasurementCatalog::get_db_table_name();
        let relate_table_name = MeasurementCatalog::get_db_relate_name();

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
