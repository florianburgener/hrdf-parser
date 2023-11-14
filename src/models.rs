use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::hrdf::Hrdf;

#[allow(unused)]
#[derive(Debug)]
pub struct Stop {
    pub id: i32,
    name: String,
    long_name: Option<String>,
    abbreviation: Option<String>,
    synonyms: Option<Vec<String>>,
    parent: RefCell<Weak<Hrdf>>,
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
            parent: RefCell::new(Weak::new()),
        }
    }

    pub fn set_parent_reference(&self, parent: &Rc<Hrdf>) {
        *self.parent.borrow_mut() = Rc::downgrade(parent);
    }

    pub fn lv95_coordinate(&self) -> Option<Rc<Lv95Coordinate>> {
        self.parent.borrow().upgrade().unwrap().lv95_stop_coordinates_index_1().get(&self.id).cloned()
    }

    pub fn wgs_coordinate(&self) -> Option<Rc<WgsCoordinate>> {
        self.parent.borrow().upgrade().unwrap().wgs_stop_coordinates_index_1().get(&self.id).cloned()
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
    pub stop_id: i32,
}

impl WgsCoordinate {
    pub fn new(latitude: f64, longitude: f64, altitude: i16, stop_id: i32) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
            stop_id,
        }
    }
}
