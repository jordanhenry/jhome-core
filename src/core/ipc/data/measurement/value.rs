use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DataValue {
    String(String),
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Serialize, Deserialize)]
pub enum Quality {
    Ok,
    Bad,
    Missing,
}

#[derive(Serialize, Deserialize)]
pub struct MeasurementValue {
    id: String,
    definition_id: String,
    data_value: DataValue,
    timestamp: u128,
    quality: Quality,
}

impl MeasurementValue {
    pub fn new(
        id: String,
        definition_id: String,
        data_value: DataValue,
        timestamp: u128,
        quality: Quality,
    ) -> MeasurementValue {
        MeasurementValue {
            id,
            definition_id,
            data_value,
            timestamp,
            quality,
        }
    }
}
