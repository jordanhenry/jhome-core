use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Composition {
    id: String,
    device_id: String,
}

impl Composition {
    pub fn new(device_id: String) -> Composition {
        Composition {
            id: Uuid::new_v4().to_string(),
            device_id,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_device_id(&self) -> &String {
        &self.device_id
    }
}
