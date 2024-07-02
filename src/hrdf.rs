use std::{error::Error, fs, path::Path, time::Instant};

use serde::{Deserialize, Serialize};

use crate::{
    constants::{CACHE_PATH, FORCE_REBUILD_CACHE},
    models::Version,
    storage::DataStorage,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hrdf {
    data_storage: DataStorage,
}

impl Hrdf {
    pub fn new(version: Version, verbose: bool) -> Result<Self, Box<dyn Error>> {
        let now = Instant::now();
        let hrdf = if Path::new(CACHE_PATH).exists() && !FORCE_REBUILD_CACHE {
            if verbose {
                println!("Reading from cache...");
            }
            Hrdf::load_from_cache()?
        } else {
            if verbose {
                println!("Building cache...");
            }
            let hrdf = Self {
                data_storage: DataStorage::new(version)?,
            };
            hrdf.build_cache()?;
            hrdf
        };
        let elapsed = now.elapsed();

        if verbose {
            println!("{:.2?}", elapsed);
        }

        Ok(hrdf)
    }

    // Getters/Setters

    pub fn data_storage(&self) -> &DataStorage {
        &self.data_storage
    }

    // Functions

    pub fn build_cache(&self) -> Result<(), Box<dyn Error>> {
        let data = bincode::serialize(&self)?;
        fs::write(CACHE_PATH, data)?;
        Ok(())
    }

    pub fn load_from_cache() -> Result<Self, Box<dyn Error>> {
        let data = fs::read(CACHE_PATH)?;
        let hrdf: Self = bincode::deserialize(&data)?;
        Ok(hrdf)
    }
}
