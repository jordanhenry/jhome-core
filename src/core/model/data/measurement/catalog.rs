use crate::core::model::data::measurement::definition::DataType;
use crate::core::model::data::measurement::definition::MeasurementDefinition;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct MeasurementCatalog {
    id: String,
    name: String,
    description: String,
    measurement_definitions: HashMap<String, MeasurementDefinition>,
}

impl MeasurementCatalog {
    pub fn new(id: String, name: String, description: String) -> MeasurementCatalog {
        MeasurementCatalog {
            id,
            name,
            description,
            measurement_definitions: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_measurement_definitions(&self) -> &HashMap<String, MeasurementDefinition> {
        &self.measurement_definitions
    }

    pub fn add_measurement_definition(
        &mut self,
        id: String,
        name: String,
        description: String,
        data_type: DataType,
        unit_id: String,
    ) {
        let definition =
            MeasurementDefinition::new(id.clone(), name, description, data_type, unit_id);
        self.measurement_definitions.insert(id, definition);
    }

    pub fn add_measurement_definitions(&mut self, mut definitions: Vec<MeasurementDefinition>) {
        while let Some(definition) = definitions.pop() {
            self.measurement_definitions
                .insert(definition.get_id().clone(), definition);
        }
    }
}

// impl Default for MeasurementCatalog {
//     fn default() -> Self {
//         Self::new(
//             Uuid::new_v4().to_string(),
//             "MyMeasurementCatalog".to_string(),
//             "".to_string(),
//         )
//     }
// }
