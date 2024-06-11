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
    println!("{:.2?}", elapsed);

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

    const N: u32 = 10;
    let before = Instant::now();

    for i in 0..N {
        test_plan_journey(&hrdf, i == 0);
    }

    let elapsed = before.elapsed();
    println!("{:.2?}", elapsed / N);

    Ok(())
}

fn test_plan_journey(hrdf: &Hrdf, verbose: bool) {
    let departure_date = NaiveDate::from_ymd_opt(2023, 2, 3).unwrap();

    // 8592688     Chancy, Les Bouveries
    // 8587031     Avully, village
    // 8508134     Bernex, Vailly
    // 8587418     Petit-Lancy, Les Esserts
    // 8592995     Petit-Lancy, Quidort
    // 8587387     Genève, Bel-Air
    // 8592910     Genève, Terrassière
    // 8587057     Genève, gare Cornavin
    // 8593189     Pont-Céard, gare
    // 8592713     Chêne-Bourg, Place Favre
    // 8588197     Sevelen, Post
    // ...
    // 8501008     Genève
    // 8501120     Lausanne

    // Chancy, Les Bouveries => Pont-Céard, gare
    // hrdf.plan_journey(8592688, 8593189, departure_date, Time::new(14, 31), verbose);

    // Chancy, Les Bouveries => Chêne-Bourg, Place Favre
    // hrdf.plan_journey(8592688, 8592713, departure_date, Time::new(14, 31), verbose);

    // Chancy, Les Bouveries => Genève, Bel-Air
    // hrdf.plan_journey(8592688, 8587387, departure_date, Time::new(14, 31), verbose);

    // Chancy, Les Bouveries => Genève, gare Cornavin
    // hrdf.plan_journey(8592688, 8587057, departure_date, Time::new(14, 31), verbose);
    hrdf.plan_journey(8592688, 8587057, departure_date, Time::new(12, 55), verbose);

    // Chancy, Les Bouveries => Genève
    // hrdf.plan_journey(8592688, 8501008, departure_date, Time::new(14, 31), verbose);

    // Chancy, Les Bouveries => Lausanne
    // hrdf.plan_journey(8592688, 8501120, departure_date, Time::new(14, 31), verbose);

    // Chancy, Les Bouveries => Sevelen, Post
    // hrdf.plan_journey(8592688, 8588197, departure_date, Time::new(14, 31), verbose); // Impossible to add these two times together!
    // hrdf.plan_journey(8592688, 8588197, departure_date, Time::new(6, 31), verbose);
}
