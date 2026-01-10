use core::str;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone)]
pub struct Animation {
    name: String,
    fps: u64,
    frames: usize,
}

impl Animation {
    const METADATA_PATH_BUILD: &str = "sk9822-led/animations/metadata/animations.json";
    const ANIMATIONS_PATH: &str = "animations/";

    pub fn new(name: &str, fps: u64, frames: usize) -> Self {
        Self {
            name: name.to_string(),
            fps,
            frames,
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Path::new(Self::METADATA_PATH_BUILD);
        let json_string = fs::read_to_string(path)?;
        let mut animations: HashMap<String, Animation> = if json_string.trim().is_empty() {
            HashMap::new()
        } else {
            serde_json::from_str(&json_string)?
        };
        animations.insert(self.name.clone(), self.clone());

        let json = serde_json::to_string_pretty(&animations)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<File, std::io::Error> {
        File::open(
            Path::new(Animation::ANIMATIONS_PATH)
                .join(self.name.clone())
                .with_extension("bin"),
        )
    }

    pub fn get_time_per_frame(&self) -> Duration {
        Duration::from_millis(1000 / self.fps)
    }

    pub fn frames(&self) -> usize {
        self.frames
    }
}
