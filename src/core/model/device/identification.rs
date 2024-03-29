use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum DeviceType {
    Sensor,
    Gateway,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Identification {
    id: String,
    name: String,
    r#type: DeviceType,
}

impl Identification {
    pub fn new(id: String, name: String, r#type: DeviceType) -> Identification {
        Identification { id, name, r#type }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}