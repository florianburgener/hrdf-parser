use serde::Serialize;
use strum_macros::EnumString;

use crate::models::Coordinates;

#[derive(Debug, Serialize)]
pub struct IsochroneCollection {
    items: Vec<Isochrone>,
    departure_stop_coord: Coordinates,
}

impl IsochroneCollection {
    pub fn new(isochrones: Vec<Isochrone>, departure_stop_coord: Coordinates) -> Self {
        Self {
            items: isochrones,
            departure_stop_coord,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Isochrone {
    polygons: Vec<Vec<Coordinates>>,
    time_limit: u32, // In minutes.
}

impl Isochrone {
    pub fn new(polygons: Vec<Vec<Coordinates>>, time_limit: u32) -> Self {
        Self {
            polygons,
            time_limit,
        }
    }
}

#[derive(Debug, EnumString, PartialEq)]
pub enum DisplayMode {
    #[strum(serialize = "circles")]
    Circles,
    #[strum(serialize = "contour_line")]
    ContourLine,
}