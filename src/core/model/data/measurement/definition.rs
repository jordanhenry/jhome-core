use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum DataType {
    String,
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MeasurementDefinition {
    id: String,
    name: String,
    description: String,
    data_type: DataType,
    unit_id: String,
}

impl MeasurementDefinition {
    pub fn new(
        id: String,
        name: String,
        description: String,
        data_type: DataType,
        unit_id: String,
    ) -> MeasurementDefinition {
        MeasurementDefinition {
            id,
            name,
            description,
            data_type,
            unit_id,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}
