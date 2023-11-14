use std::{cell::RefCell, fmt, rc::Rc};

use crate::hrdf::Hrdf;

#[allow(unused)]
pub struct Stop {
    pub id: i32,
    name: String,
    long_name: Option<String>,
    abbreviation: Option<String>,
    synonyms: Option<Vec<String>>,
    hrdf: Option<Rc<RefCell<Hrdf>>>,
}

impl Stop {
    pub fn new(
        id: i32,
        name: String,
        long_name: Option<String>,
        abbreviation: Option<String>,
        synonyms: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            name,
            long_name,
            abbreviation,
            synonyms,
            hrdf: None,
        }
    }

    pub fn lv95_coordinate(&self) -> Option<Rc<Lv95Coordinate>> {
        self.hrdf.as_ref().and_then(|hrdf| {
            hrdf.borrow().lv95_stop_coordinates_index_1().get(&self.id).cloned()
        })
    }

    pub fn set_hrdf(&mut self, hrdf: &Rc<RefCell<Hrdf>>) {
        self.hrdf = Some(Rc::clone(hrdf))
    }
}

impl fmt::Debug for Stop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stop")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Lv95Coordinate {
    easting: f64,
    northing: f64,
    altitude: i16,
    pub stop_id: i32,
}

impl Lv95Coordinate {
    pub fn new(easting: f64, northing: f64, altitude: i16, stop_id: i32) -> Self {
        Self {
            easting,
            northing,
            altitude,
            stop_id,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct WgsCoordinate {
    latitude: f64,
    longitude: f64,
    altitude: i16,
}

impl WgsCoordinate {
    pub fn new(latitude: f64, longitude: f64, altitude: i16) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
    }
}
