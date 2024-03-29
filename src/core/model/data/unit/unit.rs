use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Unit {
    id: String,
    name: String,
    symbol: String,
}

impl Unit {
    pub fn new(id: String, name: String, symbol: String) -> Unit {
        Unit { id, name, symbol }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}
