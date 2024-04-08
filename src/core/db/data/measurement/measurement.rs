use crate::core::db::{Db, Record};
use crate::core::model::data::measurement::measurement::Measurement;
use anyhow::{anyhow, Result};

impl Measurement {
    pub fn get_db_table_name() -> String {
        String::from("measurement")
    }

    pub fn get_db_relate_name() -> String {
        String::from("device_measurements")
    }

    pub async fn get(db: &Db, id: String) -> Result<Option<Measurement>> {
        let measurement: Option<Measurement> = db
            .get_db()
            .select((Measurement::get_db_table_name(), id))
            .await?;

        if let Some(measurement) = measurement {
            Ok(Some(measurement))
        } else {
            Ok(None)
        }
    }

    pub async fn get_from_relation(db: &Db, id_in: String) -> Result<Option<Vec<Measurement>>> {
        let sql = format!(
            "SELECT out FROM {} WHERE in=\"{}\";",
            Measurement::get_db_relate_name(),
            id_in
        );

        let mut ret = db.get_db().query(sql).await?;

        let mut cnt = 0;
        let mut measurements = Vec::new();
        while cnt < ret.num_statements() {
            let measurement_id: Option<String> = ret.take(cnt)?;
            let measurement_id = match measurement_id {
                Some(measurement_id) => measurement_id,
                None => {
                    cnt += 1;
                    continue;
                }
            };

            let measurement = Measurement::get(db, measurement_id).await?;
            let measurement = match measurement {
                Some(measurement) => measurement,
                None => {
                    cnt += 1;
                    continue;
                }
            };

            measurements.push(measurement);

            cnt += 1;
        }

        Ok(Some(measurements))
    }

    pub async fn push(&self, db: &Db) -> Result<String> {
        let table_name = Measurement::get_db_table_name();

        let _: Vec<Record> = db.get_db().create(table_name.clone()).content(self).await?;

        Ok(format!("{}:⟨{}⟩", table_name, self.get_id().clone()))
    }

    pub async fn relate(&self, db: &Db, id_to_relate: String) -> Result<()> {
        let table_name = Measurement::get_db_table_name();
        let relate_table_name = Measurement::get_db_relate_name();

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
