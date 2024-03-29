use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum HealthState {
    Good,
    Bad,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    state: HealthState,
    timestamp: u128,
}

impl Hello {
    pub fn new(state: HealthState, timestamp: u128) -> Hello {
        Hello { state, timestamp }
    }

    pub fn get_state(&self) -> &HealthState {
        &self.state
    }

    pub fn get_timestamp(&self) -> &u128 {
        &self.timestamp
    }
}
