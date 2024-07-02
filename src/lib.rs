mod constants;
mod debug;
mod hrdf;
mod isochrone;
mod models;
mod parsing;
mod routing;
mod service;
mod storage;
mod utils;

use std::{env, error::Error};

use debug::run_debug;
use hrdf::Hrdf;
use models::Version;
use service::run_service;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let hrdf = Hrdf::new(Version::V_5_40_41_2_0_5, true)?;

    let args: Vec<String> = env::args().collect();

    if args.get(1).map(|s| s.as_str()) == Some("serve") {
        run_service(hrdf).await;
    } else {
        run_debug(hrdf);
    }

    Ok(())
}
