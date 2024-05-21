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

    let hrdf = if Path::new(CACHED_PATH).exists() {
        let data = fs::read(CACHED_PATH)?;
        bincode::deserialize(&data).unwrap()
    } else {
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

    // println!();

    // if let Some(stop) = hrdf.stops_pk_index().get(&8587387) {
    //     println!("{:?}", stop);
    //     println!("{:?}", stop.lv95_coordinate().as_ref().unwrap());
    //     println!("{:?}", stop.wgs84_coordinate().as_ref().unwrap());
    // }

    Ok(())
}
