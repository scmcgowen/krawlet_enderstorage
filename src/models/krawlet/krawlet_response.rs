use serde::{Serialize, Deserialize};
use crate::models::enderstorage::enderstorage::EnderStorage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrawletResponse {
    success: bool,
    data: KrawletData,
    meta: KrawletMeta
}
impl KrawletResponse {
    pub fn toEnderStorages(&self) -> Vec<EnderStorage> {
        self.data.data.iter().map(|krawlet| krawlet.toEnderStorage()).collect()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct KrawletData {
    data: Vec<KrawletEnderStorage>,
    #[serde(rename="retrievedAt")]
    retrieved_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KrawletMeta {
    timestamp: String,
    elapsed: u32,
    version: String,
    #[serde(rename="requestId")]
    request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KrawletEnderStorage {
    colors: Vec<StorageColor>,
    #[serde(rename="displayName")]
    display_name: String,
}
impl KrawletEnderStorage {
    pub fn toEnderStorage(&self) -> EnderStorage {
        EnderStorage::new(self.display_name.clone(), self.colors.iter().map(|color| color.color).collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StorageColor {
    name: String,
    color: u32, // Color as a ComputerCraft-style color code, e.g. gray = 128, cyan = 512, Refer to CC's colors api for actual color numbers
}
