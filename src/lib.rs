mod hrdf;
mod models;
mod parsing;
mod routing;
mod storage;
mod utils;

use std::{error::Error, path::Path, time::Instant};

use utils::create_date_time;

use crate::hrdf::Hrdf;

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

    #[rustfmt::skip]
    println!("{}", hrdf.data_storage().journeys().journeys_by_day().values().map(|v| v.len()).sum::<usize>());
    #[rustfmt::skip]
    println!("{}", hrdf.data_storage().journeys().journeys_by_stop_id().values().map(|v| v.len()).sum::<usize>());

    println!("");

    const N: u32 = 10;
    let before = Instant::now();

    for i in 0..N {
        test_plan_journey(&hrdf, i == 0);
    }

    let elapsed = before.elapsed();
    println!("\n{:.2?}", elapsed / N);

    Ok(())
}

#[rustfmt::skip]
fn test_plan_journey(hrdf: &Hrdf, verbose: bool) {
    // 8592688     Chancy, Les Bouveries
    // 8587031     Avully, village
    // 8508134     Bernex, Vailly
    // 8587418     Petit-Lancy, Les Esserts
    // 8592995     Petit-Lancy, Quidort
    // 8587062     Genève, Jonction
    // 8587387     Genève, Bel-Air
    // 8592910     Genève, Terrassière
    // 8587057     Genève, gare Cornavin
    // 8593189     Pont-Céard, gare
    // 8592713     Chêne-Bourg, Place Favre
    // 8588197     Sevelen, Post
    // ...
    // 8501008     Genève
    // 8501120     Lausanne
    // 8768600     Paris Gare de Lyon

    // Chancy, Les Bouveries => Pont-Céard, gare
    // hrdf.plan_journey(8592688, 8593189, create_date_time(2023, 2, 3, 14, 13), verbose);

    // Chancy, Les Bouveries => Petit-Lancy, Les Esserts
    // hrdf.plan_journey(8592688, 8587418, create_date_time(2023, 2, 3, 23, 2), verbose);

    // Chancy, Les Bouveries => Genève, Bel-Air
    // hrdf.plan_journey(8592688, 8587387, create_date_time(2023, 2, 3, 14, 31), verbose);

    // Chancy, Les Bouveries => Genève, gare Cornavin
    // hrdf.plan_journey(8592688, 8587057, create_date_time(2023, 2, 3, 12, 55), verbose);
    // hrdf.plan_journey(8592688, 8587057, create_date_time(2023, 2, 3, 14, 31), verbose);
    // hrdf.plan_journey(8592688, 8587057, create_date_time(2023, 2, 3, 20, 40), verbose);
    // hrdf.plan_journey(8592688, 8587057, create_date_time(2023, 2, 3, 21, 40), verbose);

    // Chancy, Les Bouveries => Genève
    // hrdf.plan_journey(8592688, 8501008, create_date_time(2023, 2, 3, 14, 31), verbose);

    // Chancy, Les Bouveries => Lausanne
    // hrdf.plan_journey(8592688, 8501120, create_date_time(2023, 2, 3, 14, 31), verbose);
    // hrdf.plan_journey(8592688, 8501120, create_date_time(2023, 2, 3, 23, 31), verbose);

    // Chancy, Les Bouveries => Sevelen, Post
    // hrdf.plan_journey(8592688, 8588197, create_date_time(2023, 2, 1, 6, 31), verbose);
    // hrdf.plan_journey(8592688, 8588197, create_date_time(2023, 2, 1, 14, 31), verbose);
    // hrdf.plan_journey(8592688, 8588197, create_date_time(2023, 2, 1, 18, 31), verbose);

    // ...

    // Petit-Lancy, Les Esserts => Chancy, Les Bouveries
    // hrdf.plan_journey(8587418, 8592688, create_date_time(2023, 2, 3, 23, 33), verbose);

    // Genève => Chancy, Les Bouveries
    // hrdf.plan_journey(8501008, 8592688, create_date_time(2023, 2, 3, 12, 16), verbose);

    // Genève => Genève, Jonction
    // hrdf.plan_journey(8501008, 8587062, create_date_time(2023, 2, 3, 13, 25), verbose);

    // Genève, gare Cornavin => Paris Gare de Lyon
    hrdf.plan_journey(8587057, 8768600, create_date_time(2023, 2, 3, 13, 25), verbose);
}
