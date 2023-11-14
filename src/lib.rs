mod hrdf;
mod models;
mod parsing;

use std::error::Error;

use crate::hrdf::Hrdf;

pub fn run() -> Result<(), Box<dyn Error>> {
    let hrdf = Hrdf::new()?;

    println!("{} stops", hrdf.borrow().stops().len());

    if let Some(stop) = hrdf.borrow().stops_primary_index().get(&8587387) {
        println!("{:?}", stop);
        println!("{:?}", stop.borrow().lv95_coordinate().unwrap());
    }

    Ok(())
}
