use crate::core::db::{Db, Record};
use crate::core::model::system::composition::Composition;
use anyhow::Result;

impl Composition {
    pub fn get_db_table_name() -> String {
        String::from("composition")
    }

    pub fn get_db_relate_name() -> String {
        String::from("device_compositions")
    }

    pub async fn push(&self, db: &Db) -> Result<String> {
        let table_name = Composition::get_db_table_name();

        let _: Vec<Record> = db.get_db().create(table_name.clone()).content(self).await?;

        Ok(format!("{}:⟨{}⟩", table_name, self.get_id().clone()))
    }

    pub async fn relate(&self, db: &Db, id_to_relate: String) -> Result<()> {
        let table_name = Composition::get_db_table_name();
        let relate_table_name = Composition::get_db_relate_name();

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
