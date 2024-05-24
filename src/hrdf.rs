use std::{error::Error, fs, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::storage::DataStorage;

const CACHED_PATH: &str = "data.cache";

#[derive(Debug, Serialize, Deserialize)]
pub struct Hrdf {
    data_storage: Rc<DataStorage>,
}

impl Hrdf {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let instance = Self {
            data_storage: DataStorage::new()?,
        };
        Ok(instance)
    }

    pub fn data_storage(&self) -> &DataStorage {
        &self.data_storage
    }

    // ---

    fn set_data_storage_references(&self) {
        self.data_storage().set_references(&self.data_storage);
    }

    fn remove_data_storage_references(&self) {
        self.data_storage().remove_references();
    }

    pub fn build_cache(self) -> Result<Self, Box<dyn Error>> {
        self.remove_data_storage_references();

        let data = bincode::serialize(&self).unwrap();
        fs::write(CACHED_PATH, data)?;

        self.set_data_storage_references();

        Ok(self)
    }

    pub fn load_from_cache() -> Result<Hrdf, Box<dyn Error>> {
        let data = fs::read(CACHED_PATH)?;
        let hrdf: Hrdf = bincode::deserialize(&data).unwrap();

        hrdf.set_data_storage_references();

        Ok(hrdf)
    }
}
