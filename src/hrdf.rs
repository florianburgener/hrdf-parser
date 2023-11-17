use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{JourneyStop, Lv95Coordinate, Platform, Stop, WgsCoordinate},
    parsing::{self},
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    // Tables
    stops: Vec<Rc<Stop>>,
    lv95_stop_coordinates: Vec<Rc<Lv95Coordinate>>,
    wgs_stop_coordinates: Vec<Rc<WgsCoordinate>>,
    journey_stop_platforms: Vec<Rc<JourneyStop>>,
    platforms: Vec<Rc<Platform>>,

    // Indexes
    stops_primary_index: HashMap<i32, Rc<Stop>>, // Key = Haltestellennummer
    lv95_stop_coordinates_index_1: HashMap<i32, Rc<Lv95Coordinate>>, // Key = Haltestellennummer
    wgs_stop_coordinates_index_1: HashMap<i32, Rc<WgsCoordinate>>, // Key = Haltestellennummer
    journey_stop_platforms_index_1: HashMap<(i32, i32), Vec<Rc<JourneyStop>>>, // Key = (Haltestellennummer, Fahrtnummer)
    platforms_primary_index: HashMap<(i32, i32), Rc<Platform>>, // Key = (Haltestellennummer, Index der Gleistextinformation)
}

impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let _ = parsing::load_timetable_key_data();

        let stops = parsing::load_stops()?;
        let lv95_stop_coordinates = parsing::load_lv95_stop_coordinates()?;
        let wgs_stop_coordinates = parsing::load_wgs_stop_coordinates()?;
        let (journey_stop_platforms, platforms) =
            parsing::load_journey_stop_platforms_and_platforms()?;

        let stops_primary_index = parsing::create_stops_primary_index(&stops);
        let lv95_stop_coordinates_index_1 =
            parsing::create_lv95_stop_coordinates_index_1(&lv95_stop_coordinates);
        let wgs_stop_coordinates_index_1 =
            parsing::create_wgs_stop_coordinates_index_1(&wgs_stop_coordinates);
        let journey_stop_platforms_index_1 =
            parsing::create_journey_stop_platforms_index_1(&journey_stop_platforms);
        let platforms_primary_index = parsing::create_platforms_primary_index(&platforms);

        let instance = Rc::new(Self {
            // Tables
            stops,
            lv95_stop_coordinates,
            wgs_stop_coordinates,
            journey_stop_platforms,
            platforms,
            // Indexes
            stops_primary_index,
            lv95_stop_coordinates_index_1,
            wgs_stop_coordinates_index_1,
            journey_stop_platforms_index_1,
            platforms_primary_index,
        });

        Self::set_parent_references(&instance);
        Ok(instance)
    }

    fn set_parent_references(instance: &Rc<Hrdf>) {
        for stop in &instance.stops {
            stop.set_parent_reference(&instance);
        }
    }

    pub fn stops(&self) -> &Vec<Rc<Stop>> {
        &self.stops
    }

    pub fn stops_primary_index(&self) -> &HashMap<i32, Rc<Stop>> {
        &self.stops_primary_index
    }

    pub fn lv95_stop_coordinates_index_1(&self) -> &HashMap<i32, Rc<Lv95Coordinate>> {
        &self.lv95_stop_coordinates_index_1
    }

    pub fn wgs_stop_coordinates_index_1(&self) -> &HashMap<i32, Rc<WgsCoordinate>> {
        &self.wgs_stop_coordinates_index_1
    }
}
