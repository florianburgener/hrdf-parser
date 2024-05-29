use std::{error::Error, fs};

use serde::{Deserialize, Serialize};

use crate::storage::DataStorage;

const CACHED_PATH: &str = "data.cache";

#[derive(Debug, Serialize, Deserialize)]
pub struct Hrdf {
    data_storage: DataStorage,
}

impl Hrdf {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let instance = Self {
            data_storage: DataStorage::new()?,
        };
        Ok(instance)
    }

    // Getters/Setters

    pub fn data_storage(&self) -> &DataStorage {
        &self.data_storage
    }

    // Functions

    pub fn build_cache(self) -> Result<Self, Box<dyn Error>> {
        let data = bincode::serialize(&self).unwrap();
        fs::write(CACHED_PATH, data)?;

        Ok(self)
    }

    pub fn load_from_cache() -> Result<Hrdf, Box<dyn Error>> {
        let data = fs::read(CACHED_PATH)?;
        let hrdf: Hrdf = bincode::deserialize(&data).unwrap();

        Ok(hrdf)
    }
}
