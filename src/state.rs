use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Mood {
    #[default]
    Chill,
    Angry,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub mood: Mood,
}

impl State {
    fn path() -> PathBuf {
        // HOME is unset by default on native Windows (only USERPROFILE is guaranteed);
        // fall back to the OS temp dir if neither is available.
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| std::env::temp_dir().display().to_string());
        PathBuf::from(home).join(".mommy.state")
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        match fs::read_to_string(Self::path()) {
            Ok(contents) => Ok(serde_json::from_str(&contents).unwrap_or_default()),
            Err(_) => Ok(Self::default()),
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        fs::write(Self::path(), serde_json::to_string(self)?)?;
        Ok(())
    }
}
