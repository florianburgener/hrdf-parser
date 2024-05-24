mod hrdf;
mod models;
mod parsing;
mod routing;
mod storage;
mod utils;

use std::{error::Error, path::Path, time::Instant};

use chrono::NaiveDate;

use crate::{hrdf::Hrdf, models::Time};

pub fn run() -> Result<(), Box<dyn Error>> {
    const CACHED_PATH: &str = "data.cache";

    let now = Instant::now();
    let hrdf = if Path::new(CACHED_PATH).exists() && false {
        println!("Reading from cache...");
        Hrdf::load_from_cache()?
    } else {
        println!("Building cache...");
        Hrdf::new()?.build_cache()?
    };
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    // sleep(Duration::from_secs(30));

    println!();
    println!("------------------------------------------------------------------------------------------------");
    println!("--- Tests");
    println!("------------------------------------------------------------------------------------------------");
    println!();

    println!("{} journeys", hrdf.data_storage().journeys().rows().len());
    println!("{} platforms", hrdf.data_storage().platforms().rows().len());
    println!("{} stops", hrdf.data_storage().stops().rows().len());

    let departure_date = NaiveDate::from_ymd_opt(2023, 02, 03).unwrap();
    // Chancy=8592688
    hrdf.plan_trip(8587418, 8508134, departure_date, Time::new(14, 12));

    Ok(())
}
