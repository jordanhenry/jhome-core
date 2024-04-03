use crate::core::db::{Db, Record};
use crate::core::model::data::measurement::definition::MeasurementDefinition;
use anyhow::{anyhow, Result};

impl MeasurementDefinition {
    pub fn get_db_table_name() -> String {
        String::from("measurement_definition")
    }

    pub fn get_db_relate_name() -> String {
        String::from("measurement_definitions")
    }

    pub async fn get(db: &Db, id: String) -> Result<Option<MeasurementDefinition>> {
        let measurement_def: Option<MeasurementDefinition> = db
            .get_db()
            .select((MeasurementDefinition::get_db_table_name(), id))
            .await?;

        if let Some(measurement_def) = measurement_def {
            Ok(Some(measurement_def))
        } else {
            Ok(None)
        }
    }

    pub async fn get_from_relation(
        db: &Db,
        id_in: String,
    ) -> Result<Option<Vec<MeasurementDefinition>>> {
        let sql = format!(
            "SELECT out FROM {} WHERE in=\"{}\";",
            MeasurementDefinition::get_db_relate_name(),
            id_in
        );

        let mut ret = db.get_db().query(sql).await?;

        let mut cnt = 0;
        let mut measurement_definitions = Vec::new();
        while cnt < ret.num_statements() {
            let measurement_def_id: Option<String> = ret.take(cnt)?;
            let measurement_def_id = match measurement_def_id {
                Some(measurement_def_id) => measurement_def_id,
                None => {
                    cnt += 1;
                    continue;
                }
            };

            let measurement_def = MeasurementDefinition::get(db, measurement_def_id).await?;
            let measurement_def = match measurement_def {
                Some(measurement_def) => measurement_def,
                None => {
                    cnt += 1;
                    continue;
                }
            };

            measurement_definitions.push(measurement_def);

            cnt += 1;
        }

        Ok(Some(measurement_definitions))
    }

    pub async fn push(&self, db: &Db) -> Result<String> {
        let table_name = MeasurementDefinition::get_db_table_name();

        let _: Vec<Record> = db.get_db().create(table_name.clone()).content(self).await?;

        Ok(format!("{}:⟨{}⟩", table_name, self.get_id().clone()))
    }

    pub async fn relate(&self, db: &Db, id_to_relate: String) -> Result<()> {
        let table_name = MeasurementDefinition::get_db_table_name();
        let relate_table_name = MeasurementDefinition::get_db_relate_name();

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
