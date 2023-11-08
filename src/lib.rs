mod hrdf;
mod models;
mod parser;

use std::error::Error;

use crate::hrdf::Hrdf;

pub fn run() -> Result<(), Box<dyn Error>> {
    let hrdf = Hrdf::new()?;

    println!("Stops:");

    for stop in &hrdf.stops {
        println!("{:?}", stop);
    }

    Ok(())
}
