mod hrdf;
mod models;
mod parsing;

use std::error::Error;

use crate::hrdf::Hrdf;

pub fn run() -> Result<(), Box<dyn Error>> {
    let hrdf = Hrdf::new()?;

    println!("{} stops", hrdf.stops.len());

    Ok(())
}
