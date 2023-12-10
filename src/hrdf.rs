use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{JourneyPlatform, Platform, Stop, TimetableKeyData, Attribute, Holiday},
    parsing,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    attributes: Vec<Rc<Attribute>>,
    attributes_primary_index: HashMap<String, Rc<Attribute>>, // Key = Attribute.id

    holidays: Vec<Rc<Holiday>>,

    journey_platform: Vec<Rc<JourneyPlatform>>,
    journey_platform_primary_index: HashMap<(i32, i64), Vec<Rc<JourneyPlatform>>>, // Key = (Stop.id, Platform.id)

    platforms: Vec<Rc<Platform>>,
    platforms_primary_index: HashMap<i64, Rc<Platform>>, // Key = Platform.id

    stops: Vec<Rc<Stop>>,
    stops_primary_index: HashMap<i32, Rc<Stop>>, // Key = Stop.id

    timetable_key_data: TimetableKeyData,
}

impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let (attributes, attributes_primary_index) = parsing::load_attributes()?;
        let holidays = parsing::load_holidays()?;
        print!("{:?}", holidays);
        let (journey_platform, journey_platform_primary_index, platforms, platforms_primary_index) =
            parsing::load_journey_platform_and_platforms()?;
        let (stops, stops_primary_index) = parsing::load_stops()?;
        let timetable_key_data = parsing::load_timetable_key_data()?;

        let instance = Rc::new(Self {
            attributes,
            holidays,
            attributes_primary_index,
            journey_platform,
            journey_platform_primary_index,
            platforms,
            platforms_primary_index,
            stops,
            stops_primary_index,
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

    pub fn stops_primary_index(&self) -> &HashMap<i32, Rc<Stop>> {
        &self.stops_primary_index
    }
}
