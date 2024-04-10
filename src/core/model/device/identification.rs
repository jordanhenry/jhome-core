use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DeviceType {
    Sensor,
    Gateway,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_type(&self) -> &DeviceType {
        &self.r#type
    }
}
