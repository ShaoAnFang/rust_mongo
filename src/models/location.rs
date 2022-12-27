use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub add: String,
    pub description: String,
    pub region: String,
}

impl Default for Location {
    fn default() -> Location {
        Location {
            name: "".to_string(),
            add: "".to_string(),
            description: "".to_string(),
            region: "".to_string(),
        }
    }
}

