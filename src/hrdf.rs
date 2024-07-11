use std::{
    error::Error,
    fs::{self, File},
    io::{BufReader, Cursor},
    path::Path,
    time::Instant,
};

use crate::{models::Version, storage::DataStorage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use url::Url;
use zip::ZipArchive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Hrdf {
    data_storage: DataStorage,
}

impl Hrdf {
    pub async fn new(
        version: Version,
        url_or_path: &str,
        verbose: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let now = Instant::now();

        let unique_filename = format!("{:x}", Sha256::digest(url_or_path.as_bytes()));
        let cache_path = format!("{unique_filename}.cache");

        // TODO: FORCE_REBUILD_CACHE as variable.
        let hrdf = if Path::new(&cache_path).exists() {
            // Loading from cache.
            if verbose {
                println!("Reading from cache...");
            }

            // If loading from cache fails, None is returned.
            Hrdf::load_from_cache(&cache_path).ok()
        } else {
            // No loading from cache.
            None
        };

        let hrdf = if let Some(hrdf) = hrdf {
            // The cache has been loaded without error.
            hrdf
        } else {
            // The cache must be built.
            // If cache loading has failed, the cache must be rebuilt.
            let compressed_data_path = if Url::parse(url_or_path).is_ok() {
                let compressed_data_path = format!("/tmp/{unique_filename}.zip");

                if !Path::new(&compressed_data_path).exists() {
                    // The data must be downloaded.
                    println!("Downloading data...");
                    let response = reqwest::get(url_or_path).await?;
                    let mut file = std::fs::File::create(&compressed_data_path)?;
                    let mut content = Cursor::new(response.bytes().await?);
                    std::io::copy(&mut content, &mut file)?;
                }

                compressed_data_path
            } else {
                url_or_path.to_string()
            };

            let decompressed_data_path = format!("/tmp/{unique_filename}");

            if !Path::new(&decompressed_data_path).exists() {
                // The data must be decompressed.
                println!("Unzipping archive...");
                let file = File::open(&compressed_data_path)?;
                let mut archive = ZipArchive::new(BufReader::new(file))?;
                archive.extract(&decompressed_data_path)?;
            }

            if verbose {
                println!("{decompressed_data_path}");
                println!("Building cache...");
            }

            let hrdf = Self {
                data_storage: DataStorage::new(version, &decompressed_data_path)?,
            };

            hrdf.build_cache(&cache_path)?;
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

    pub fn build_cache(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let data = bincode::serialize(&self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn load_from_cache(path: &str) -> Result<Self, Box<dyn Error>> {
        let data = fs::read(path)?;
        let hrdf: Self = bincode::deserialize(&data)?;
        Ok(hrdf)
    }
}
