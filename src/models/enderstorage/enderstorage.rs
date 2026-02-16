use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnderStorage { // Minimal representation of an EnderStorage for the selector. This is all it uses
    label: String,
    colors: Vec<u32>,
}

impl EnderStorage {
    pub fn new(label: String, colors: Vec<u32>) -> Self {
        EnderStorage { label, colors }
    }
}
