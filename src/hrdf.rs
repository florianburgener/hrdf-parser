use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Lv95Coordinate, Stop},
    parsing::{self, ColumnDefinition, ExpectedType, FileParser, SingleConfigurationRowParser},
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    lv95_stop_coordinates: Vec<Rc<Lv95Coordinate>>,
    lv95_stop_coordinates_index_1: Rc<HashMap<i32, Rc<Lv95Coordinate>>>,
    // wgs_stop_coordinates: Vec<WgsCoordinate>,
    stops: Vec<Rc<Stop>>,
    stops_primary_index: HashMap<i32, Rc<Stop>>,
}

impl Hrdf {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let lv95_stop_coordinates = Self::load_lv95_stop_coordinates()?;
        let lv95_stop_coordinates_index_1 = Rc::new(Self::create_lv95_stop_coordinates_index_1(
            &lv95_stop_coordinates,
        ));
        let stops = Self::load_stops(&lv95_stop_coordinates_index_1)?;
        let stops_primary_index = Self::create_stops_primary_index(&stops);

        let instance = Self {
            lv95_stop_coordinates,
            lv95_stop_coordinates_index_1,
            stops,
            stops_primary_index,
        };

        Ok(instance)
    }

    pub fn stops(&self) -> &Vec<Rc<Stop>> {
        &self.stops
    }

    pub fn stops_primary_index(&self) -> &HashMap<i32, Rc<Stop>> {
        &self.stops_primary_index
    }

    // BFKOORD_LV95 (BF = BAHNHOF)
    fn load_lv95_stop_coordinates() -> Result<Vec<Rc<Lv95Coordinate>>, Box<dyn Error>> {
        let row_configuration = vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 18, ExpectedType::Float),
            ColumnDefinition::new(20, 29, ExpectedType::Float),
            ColumnDefinition::new(31, 36, ExpectedType::Integer16),
        ];
        let row_parser = SingleConfigurationRowParser::new(row_configuration);
        let file_parser = FileParser::new("data/BFKOORD_LV95", Box::new(row_parser))?;

        let lv95_stop_coordinates = file_parser
            .iter()
            .map(|mut values| {
                let stop_id = i32::from(values.remove(0));
                let easting = f64::from(values.remove(0));
                let northing = f64::from(values.remove(0));
                let altitude = i16::from(values.remove(0));

                Rc::new(Lv95Coordinate::new(easting, northing, altitude, stop_id))
            })
            .collect();

        Ok(lv95_stop_coordinates)
    }

    fn create_lv95_stop_coordinates_index_1(
        stops: &Vec<Rc<Lv95Coordinate>>,
    ) -> HashMap<i32, Rc<Lv95Coordinate>> {
        let lv95_stop_coordinates_primary_index: HashMap<i32, Rc<Lv95Coordinate>> =
            stops.iter().fold(HashMap::new(), |mut acc, coordinate| {
                acc.insert(coordinate.stop_id, Rc::clone(coordinate));
                acc
            });
        lv95_stop_coordinates_primary_index
    }

    // BAHNHOF
    fn load_stops(
        lv95_stop_coordinates_index_1: &Rc<HashMap<i32, Rc<Lv95Coordinate>>>,
    ) -> Result<Vec<Rc<Stop>>, Box<dyn Error>> {
        let row_configuration = vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ];
        let row_parser = SingleConfigurationRowParser::new(row_configuration);
        let file_parser = FileParser::new("data/BAHNHOF", Box::new(row_parser))?;

        let stops = file_parser
            .iter()
            .map(|mut values| {
                let id = i32::from(values.remove(0));
                let raw_name = String::from(values.remove(0));

                let parsed_name = parsing::parse_stop_name(raw_name);

                let name = parsed_name.get(&1).unwrap()[0].clone();
                let long_name = parsed_name.get(&2).map(|x| x[0].clone());
                let abbreviation = parsed_name.get(&3).map(|x| x[0].clone());
                let synonyms = parsed_name.get(&4).cloned();

                Rc::new(Stop::new(
                    id,
                    name,
                    long_name,
                    abbreviation,
                    synonyms,
                    Rc::clone(lv95_stop_coordinates_index_1),
                ))
            })
            .collect();

        Ok(stops)
    }

    fn create_stops_primary_index(stops: &Vec<Rc<Stop>>) -> HashMap<i32, Rc<Stop>> {
        let stops_primary_index: HashMap<i32, Rc<Stop>> =
            stops.iter().fold(HashMap::new(), |mut acc, stop| {
                acc.insert(stop.id, Rc::clone(stop));
                acc
            });
        stops_primary_index
    }

    // // BFKOORD_WGS (BF = BAHNHOF)
    // fn load_wgs_stop_coordinates() -> Result<Vec<WgsCoordinate>, Box<dyn Error>> {
    //     let row_configuration = vec![
    //         ColumnDefinition::new(1, 7, ExpectedType::Integer32),
    //         ColumnDefinition::new(9, 18, ExpectedType::Float),
    //         ColumnDefinition::new(20, 29, ExpectedType::Float),
    //         ColumnDefinition::new(31, 36, ExpectedType::Integer16),
    //     ];
    //     let row_parser = SingleConfigurationRowParser::new(row_configuration);
    //     let file_parser = FileParser::new("data/BFKOORD_WGS", Box::new(row_parser))?;

    //     let wgs_stop_coordinates = file_parser
    //         .iter()
    //         .map(|mut values| {
    //             let longitude = f64::from(values.remove(0));
    //             let latitude = f64::from(values.remove(0));
    //             let altitude = i16::from(values.remove(0));

    //             WgsCoordinate::new(latitude, longitude, altitude)
    //         })
    //         .collect();

    //     Ok(wgs_stop_coordinates)
    // }
}
