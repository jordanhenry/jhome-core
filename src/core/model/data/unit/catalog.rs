use crate::core::model::data::unit::unit::Unit;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UnitCatalog {
    id: String,
    name: String,
    description: String,
    units: HashMap<String, Unit>,
}

impl UnitCatalog {
    pub fn new(id: String, name: String, description: String) -> UnitCatalog {
        UnitCatalog {
            id,
            name,
            description,
            units: HashMap::new(),
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

    pub fn get_units(&self) -> &HashMap<String, Unit> {
        &self.units
    }

    pub fn add_unit_definition(&mut self, id: String, name: String, symbol: String) {
        let definition = Unit::new(id.clone(), name, symbol);
        self.units.insert(id, definition);
    }

    pub fn add_unit_definitions(&mut self, mut definitions: Vec<Unit>) {
        while let Some(definition) = definitions.pop() {
            self.units.insert(definition.get_id().clone(), definition);
        }
    }
}
