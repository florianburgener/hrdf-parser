mod hrdf;
mod models;
mod parsing;
mod routing;
mod storage;
mod utils;

use std::{error::Error, fs, path::Path, rc::Rc, thread::sleep, time::Duration};

use crate::{hrdf::Hrdf, models::Time};

pub fn run() -> Result<(), Box<dyn Error>> {
    const CACHED_PATH: &str = "data.cache";

    let hrdf = if Path::new(CACHED_PATH).exists() && false {
        println!("Reading from cache...");
        let data = fs::read(CACHED_PATH)?;
        let hrdf: Rc<Hrdf> = bincode::deserialize(&data).unwrap();
        hrdf.set_references(&hrdf);
        hrdf
    } else {
        println!("Building cache...");
        let hrdf = Hrdf::new()?;
        hrdf.remove_references();

        let data = bincode::serialize(&hrdf).unwrap();
        fs::write(CACHED_PATH, data)?;

        hrdf
    };
    sleep(Duration::from_secs(30));

    println!();
    println!("------------------------------------------------------------------------------------------------");
    println!("--- Tests");
    println!("------------------------------------------------------------------------------------------------");
    println!();

    println!("{} journeys", hrdf.journeys().rows().len());
    println!("{} platforms", hrdf.platforms().rows().len());
    println!("{} stops", hrdf.stops().rows().len());

    hrdf.plan_trip(8592688, 8508134, Time::new(14, 12));

    Ok(())
}
