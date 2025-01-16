use std::{
    fs::File,
    io::{Read, Write},
};

use serde::{Deserialize, Serialize};

const STORAGE_FILE: &str = "./storage.json";

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub rpc_urls: Vec<String>,
}

impl Storage {
    pub fn load() -> Self {
        if let Ok(mut file) = File::open(STORAGE_FILE) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap_or_else(|_| Storage { rpc_urls: vec![] })
        } else {
            Storage { rpc_urls: vec![] }
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(&self).unwrap();
        let mut file = File::create(STORAGE_FILE).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn get_rpc_url(&self, index: usize) -> Option<&String> {
        self.rpc_urls.get(index)
    }
}
