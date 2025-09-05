use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Cache {
    data: HashMap<String, Value>,
    path: String,
}

impl Cache {
    /// Load cache from a JSON file, or create an empty cache if the file doesn't exist
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        if path.as_ref().exists() {
            let content = fs::read_to_string(&path)?;
            let data: HashMap<String, Value> =
                serde_json::from_str(&content).unwrap_or_else(|_| HashMap::new());
            Ok(Self { data, path: path_str })
        } else {
            Ok(Self {
                data: HashMap::new(),
                path: path_str,
            })
        }
    }

    /// Get a value for a key
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    /// Set a value for a key and save immediately
    pub fn set(&mut self, key: &str, value: Value) -> io::Result<()> {
        self.data.insert(key.to_string(), value);
        self.save()
    }

    /// Save the cache to the JSON file
    pub fn save(&self) -> io::Result<()> {
        let content = serde_json::to_string_pretty(&self.data)?;
        let mut file = File::create(&self.path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Check if a key exists
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}