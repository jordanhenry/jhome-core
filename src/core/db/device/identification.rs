use crate::core::db::{Db, Record};
use crate::core::model::device::identification::{self, Identification};
use anyhow::{anyhow, Result};

impl Identification {
    pub fn get_db_table_name() -> String {
        String::from("identification")
    }

    pub fn get_db_relate_name() -> String {
        String::from("device_identification")
    }

    pub async fn get(db: &Db, id: String) -> Result<Identification> {
        let table_name = Identification::get_db_table_name();

        let identification: Option<Identification> = db.get_db().select((table_name, id)).await?;

        if let Some(identification) = identification {
            Ok(identification)
        } else {
            Err(anyhow!("Identification not found"))
        }
    }

    pub async fn get_from_relation(db: &Db, id_in: String) -> Result<Identification> {
        let sql = format!(
            "SELECT out FROM {} WHERE in=\"{}\";",
            Identification::get_db_relate_name(),
            id_in
        );

        let mut ret = db.get_db().query(sql).await?;

        let identification_id: Option<String> = ret.take(0)?;
        let identification_id = match identification_id {
            Some(identification_id) => identification_id,
            None => return Err(anyhow!("Identification not found")),
        };

        Identification::get(db, identification_id).await
    }

    pub async fn push(&self, db: &Db) -> Result<String> {
        let table_name = Identification::get_db_table_name();

        let _: Vec<Record> = db.get_db().create(table_name.clone()).content(self).await?;

        Ok(format!("{}:⟨{}⟩", table_name, self.get_id().clone()))
    }

    pub async fn relate(&self, db: &Db, id_to_relate: String) -> Result<()> {
        let table_name = Identification::get_db_table_name();
        let relate_table_name = Identification::get_db_relate_name();

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
