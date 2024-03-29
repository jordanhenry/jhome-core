use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Who {
    All,
    Id(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum What {
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhoAreYou {
    who: Who,
    what: What,
}

impl WhoAreYou {
    pub fn new(who: Who, what: What) -> WhoAreYou {
        WhoAreYou { who, what }
    }

    pub fn get_who(&self) -> &Who {
        &self.who
    }

    pub fn get_what(&self) -> &What {
        &self.what
    }
}
