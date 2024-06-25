mod debug;
mod hrdf;
mod models;
mod parsing;
mod routing;
mod service;
mod storage;
mod utils;

use std::{env, error::Error, path::Path, time::Instant};

use debug::run_debug;
use hrdf::Hrdf;
use service::run_service;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let hrdf = load_hrdf()?;

    let args: Vec<String> = env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("serve") {
        run_service(hrdf).await;
    } else {
        run_debug(hrdf);
    }

    Ok(())
}

pub fn load_hrdf() -> Result<Hrdf, Box<dyn Error>> {
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

    Ok(hrdf)
}
