use serde::{Deserialize, Serialize};
use core::str;
use std::fs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct Animation {
    name: String,
    fps: u64,
    frames: usize,
}

impl Animation {
    const METADATA_PATH: &str = "sk9822-led/animations/metadata/animations.json";
    pub fn new(name: &str, fps: u64, frames: usize) -> Self {
        Self {
            name: name.to_string(),
            fps,
            frames,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let json_string = fs::read_to_string(Self::METADATA_PATH)?;
        let mut animations: HashMap<String, Animation> = if json_string.trim().is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&json_string)?
        };
        animations.insert(self.name.clone(), self.clone());

        let json = serde_json::to_string_pretty(&animations)?;
        fs::write(Self::METADATA_PATH, json)?;
        Ok(())
    }
}