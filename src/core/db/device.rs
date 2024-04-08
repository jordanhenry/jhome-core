use crate::core::db::{Db, Record};
use crate::core::model::data::measurement::catalog::MeasurementCatalog;
use crate::core::model::data::measurement::measurement::Measurement;
use crate::core::model::device::identification::Identification;
use crate::core::model::device::DeviceModel;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Id, Thing};

pub mod identification;

#[derive(Debug, Serialize, Deserialize)]
struct DeviceModelDb {
    id: Thing,
}

impl DeviceModel {
    pub fn get_db_table_name() -> String {
        String::from("device")
    }

    pub async fn get_all(db: &Db) -> Result<Vec<DeviceModel>> {
        //Get device list
        let device_list: Vec<DeviceModelDb> =
            db.get_db().select(DeviceModel::get_db_table_name()).await?;

        let mut devices: Vec<DeviceModel> = Vec::new();
        for device_id in device_list.iter() {
            //Identification
            let identification =
                Identification::get_from_relation(db, device_id.id.to_string()).await?;

            //Measurement Catalog
            let measurement_catalog =
                MeasurementCatalog::get_from_relation(db, device_id.id.to_string()).await?;

            //Measurment
            let measurements = Measurement::get_from_relation(db, device_id.id.to_string()).await?;

            //Unit catalog
            //Device composition

            if let Some(identification) = identification {
                let mut device = DeviceModel::new(
                    identification.get_id().clone(),
                    identification.get_name().clone(),
                    identification.get_type().clone(),
                );

                if let Some(measurement_catalog) = measurement_catalog {
                    device.set_measurement_catalog(measurement_catalog);
                }

                if let Some(measurements) = measurements {
                    device.set_measurements(measurements);
                }

                devices.push(device);
            }
        }

        Ok(devices)
    }

    pub fn get_device_table_id(&self) -> String {
        let table_name = DeviceModel::get_db_table_name();
        format!("{}:⟨{}⟩", table_name, self.get_device_id())
    }

    pub async fn is_pushed(db: &Db, device_id: String) -> Result<bool> {
        let device_table_name = DeviceModel::get_db_table_name();
        let device_table_id = format!("{}:⟨{}⟩", device_table_name, device_id);

        let device: Option<DeviceModelDb> =
            db.get_db().select((device_table_name, device_id)).await?;

        if let Some(device) = device {
            if device.id.to_string().eq(&device_table_id) {
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub async fn push(&self, db: &Db) -> Result<String> {
        //Device
        let table_name = DeviceModel::get_db_table_name();

        let _: Vec<Record> = db
            .get_db()
            .create(table_name.clone())
            .content(DeviceModelDb {
                id: Thing {
                    tb: table_name.clone(),
                    id: Id::String(self.get_device_id().clone()),
                },
            })
            .await?;

        let device_table_id = self.get_device_table_id();

        //Identification
        let _ = self.get_identification().push(db).await?;
        self.get_identification()
            .relate(db, device_table_id.clone())
            .await?;

        //Measurement catalog
        if let Some(measurement_catalog) = self.get_measurement_catalog() {
            let _ = measurement_catalog.push(db).await?;
            measurement_catalog
                .relate(db, device_table_id.clone())
                .await?;
        }

        //Measurement
        if let Some(measurements) = self.get_measurements() {
            for (_, measurement) in measurements.iter() {
                let _ = measurement.push(db).await?;
                measurement.relate(db, device_table_id.clone()).await?;
            }
        }

        //Unit catalog
        if let Some(unit_catalog) = self.get_unit_catalog() {
            let _ = unit_catalog.push(db).await?;
            unit_catalog.relate(db, device_table_id.clone()).await?;
        }

        //Composition
        if let Some(composition) = self.get_device_composition() {
            for (_, composition) in composition.iter() {
                let _ = composition.push(db).await?;
                composition.relate(db, device_table_id.clone()).await?;
            }
        }

        Ok(device_table_id)
    }
}
