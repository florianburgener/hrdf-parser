use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{JourneyPlatform, Platform, Stop, TimetableKeyData},
    parsing,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    journey_platform: Vec<Rc<JourneyPlatform>>,
    journey_platform_index: HashMap<(i32, i32), Vec<Rc<JourneyPlatform>>>, // Key = (Haltestellennummer, Fahrtnummer)

    platforms: Vec<Rc<Platform>>,
    platforms_index: HashMap<(i32, i32), Rc<Platform>>, // Key = (Haltestellennummer, Index der Gleistextinformation)

    stops: Vec<Rc<Stop>>,
    stops_index: HashMap<i32, Rc<Stop>>, // Key = Haltestellennummer

    timetable_key_data: TimetableKeyData,
}

impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let (journey_platform, journey_platform_index, platforms, platforms_index) =
            parsing::load_journey_stop_platforms_and_platforms()?;
        let (stops, stops_index) = parsing::load_stops()?;
        let timetable_key_data = parsing::load_timetable_key_data()?;

        let instance = Rc::new(Self {
            journey_platform,
            journey_platform_index,
            platforms,
            platforms_index,
            stops,
            stops_index,
            timetable_key_data,
        });

        // Self::set_parent_references(&instance);
        Ok(instance)
    }

    // fn set_parent_references(instance: &Rc<Hrdf>) {
    //     for stop in &instance.stops {
    //         stop.set_parent_reference(&instance);
    //     }
    // }

    pub fn platforms(&self) -> &Vec<Rc<Platform>> {
        return &self.platforms;
    }

    pub fn stops(&self) -> &Vec<Rc<Stop>> {
        &self.stops
    }

    pub fn stops_index(&self) -> &HashMap<i32, Rc<Stop>> {
        &self.stops_index
    }
}
