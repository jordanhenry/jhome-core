use crate::core::db::{Db, Record};
use crate::core::model::data::measurement::catalog::MeasurementCatalog;
use crate::core::model::data::measurement::definition::MeasurementDefinition;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::measurement;

#[derive(Debug, Serialize, Deserialize)]
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

    pub async fn get(db: &Db, id: String) -> Result<Option<MeasurementCatalog>> {
        let catalog: Option<MeasurementCatalogDb> = db
            .get_db()
            .select((MeasurementCatalog::get_db_table_name(), id))
            .await?;

        let catalog = match catalog {
            Some(catalog) => catalog,
            None => return Ok(None),
        };

        let mut catalog = MeasurementCatalog::new(catalog.id, catalog.name, catalog.description);

        let measurement_definitions =
            MeasurementDefinition::get_from_relation(db, catalog.get_id().clone()).await?;

        if let Some(measurement_definitions) = measurement_definitions {
            catalog.add_measurement_definitions(measurement_definitions);
        }

        Ok(Some(catalog))
    }

    pub async fn get_from_relation(db: &Db, id_in: String) -> Result<Option<MeasurementCatalog>> {
        let sql = format!(
            "SELECT out FROM {} WHERE in=\"{}\";",
            MeasurementCatalog::get_db_relate_name(),
            id_in
        );

        let mut ret = db.get_db().query(sql).await?;

        let catalog_id: Option<String> = ret.take(0)?;
        let catalog_id = match catalog_id {
            Some(catalog_id) => catalog_id,
            None => return Ok(None),
        };

        MeasurementCatalog::get(db, catalog_id).await
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
