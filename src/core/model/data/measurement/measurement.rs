use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Measurement {
    id: String,
    definition_id: String,
}

impl Measurement {
    pub fn new(id: String, definition_id: String) -> Measurement {
        Measurement { id, definition_id }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}
