use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::model::data::measurement::catalog::MeasurementCatalog;
use crate::core::model::data::measurement::measurement::Measurement;
use crate::core::model::data::unit::catalog::UnitCatalog;
use crate::core::model::device::identification::{DeviceType, Identification};
use crate::core::model::system::composition::Composition;

pub mod identification;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceModel {
    device_identification: Identification,
    measurement_catalog: Option<MeasurementCatalog>,
    measurements: Option<HashMap<String, Measurement>>,
    unit_catalog: Option<UnitCatalog>,
    device_composition: Option<HashMap<String, Composition>>,
}

impl DeviceModel {
    pub fn new(id: String, name: String) -> DeviceModel {
        DeviceModel {
            device_identification: Identification::new(id, name, DeviceType::Gateway),
            measurement_catalog: None,
            measurements: None,
            unit_catalog: None,
            device_composition: None,
        }
    }

    pub fn load_from_json(json: String) -> Result<DeviceModel> {
        let gateway = serde_json::from_str::<DeviceModel>(&json)?;
        Ok(gateway)
    }

    pub fn get_device_id(&self) -> &String {
        self.device_identification.get_id()
    }

    pub fn get_identification(&self) -> &Identification {
        &self.device_identification
    }

    pub fn get_measurements(&self) -> &Option<HashMap<String, Measurement>> {
        &self.measurements
    }

    pub fn get_measurement_catalog(&self) -> &Option<MeasurementCatalog> {
        &self.measurement_catalog
    }

    pub fn get_mut_measurement_catalog(&mut self) -> &mut Option<MeasurementCatalog> {
        &mut self.measurement_catalog
    }

    pub fn load_measurement_catalog_from_json(&mut self, json: String) -> Result<()> {
        let measurement_catalog = serde_json::from_str::<MeasurementCatalog>(&json)?;
        self.measurement_catalog = Some(measurement_catalog);
        Ok(())
    }

    pub fn get_unit_catalog(&self) -> &Option<UnitCatalog> {
        &self.unit_catalog
    }

    pub fn get_mut_unit_catalog(&mut self) -> &mut Option<UnitCatalog> {
        &mut self.unit_catalog
    }

    pub fn get_device_composition(&self) -> &Option<HashMap<String, Composition>> {
        &self.device_composition
    }

    pub fn load_unit_catalog_from_json(&mut self, json: String) -> Result<()> {
        let unit_catalog = serde_json::from_str::<UnitCatalog>(&json)?;
        self.unit_catalog = Some(unit_catalog);
        Ok(())
    }

    pub fn add_device_composition(&mut self, composition: Composition) {
        match self.device_composition {
            None => {
                let mut device_composition = HashMap::new();
                device_composition.insert(composition.get_device_id().clone(), composition);
                self.device_composition = Some(device_composition);
            }
            Some(ref mut device_composition) => {
                device_composition
                    .entry(composition.get_device_id().clone())
                    .or_insert(composition);
            }
        }
    }
}
