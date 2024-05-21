mod hrdf;
mod models;
mod parsing;
mod storage;

use std::{
    error::Error,
    fs::{self},
    path::Path,
};

use crate::hrdf::Hrdf;

pub fn run() -> Result<(), Box<dyn Error>> {
    const CACHED_PATH: &str = "data.cache";

    // TODO: remove "&& false".
    let hrdf = if Path::new(CACHED_PATH).exists() && false {
        println!("Reading from cache...");
        let data = fs::read(CACHED_PATH)?;
        bincode::deserialize(&data).unwrap()
    } else {
        println!("Building cache...");
        let hrdf = Hrdf::new()?;

        let data = bincode::serialize(&hrdf).unwrap();
        fs::write(CACHED_PATH, data)?;

        hrdf
    };

    println!();
    println!("------------------------------------------------------------------------------------------------");
    println!("--- Tests");
    println!("------------------------------------------------------------------------------------------------");
    println!();

    println!("{} journeys", hrdf.journeys().rows().len());
    println!("{} platforms", hrdf.platforms().rows().len());
    println!("{} stops", hrdf.stops().rows().len());

    Ok(())
}
