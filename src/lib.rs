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
    let hrdf = if Path::new(CACHED_PATH).exists() && true {
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

    println!(
        "{} journeys",
        hrdf.data_storage().journeys().entries().len()
    );
    // println!("{} platforms", hrdf.data_storage().platforms().items().len());
    // println!("{} stops", hrdf.data_storage().stops().items().len());

    println!("");
    let departure_date = NaiveDate::from_ymd_opt(2023, 02, 03).unwrap();
    // 8592688     Chancy, Les Bouveries
    // 8587031     Avully, village
    // 8508134     Bernex, Vailly
    // 8587418     Petit-Lancy, Les Esserts
    // 8592995     Petit-Lancy, Quidort
    // 8587387     Genève, Bel-Air
    // 8592910     Genève, Terrassière
    // 8587057     Genève, gare Cornavin
    // 8593189     Pont-Céard, gare
    // ...
    // 8501008     Genève

    let before = Instant::now();
    for _ in 0..10 {
        hrdf.plan_journey(8592688, 8593189, departure_date, Time::new(14, 31), false);
        // hrdf.plan_journey(8592688, 8587387, departure_date, Time::new(14, 31));
        // hrdf.plan_journey(8592688, 8587057, departure_date, Time::new(14, 31));
    }
    hrdf.plan_journey(8592688, 8593189, departure_date, Time::new(14, 31), true);
    println!("{:.2?}", before.elapsed() / 10);

    Ok(())
}
