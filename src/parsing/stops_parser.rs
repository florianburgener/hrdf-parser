// BAHNHOF, BHKOORD_LV95, BHKOORD_WGS
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::models::{Coordinate, CoordinateType, Stop};

use super::{ColumnDefinition, ExpectedType, FileParser, SingleConfigurationRowParser};

pub fn load_stops() -> Result<(Vec<Rc<Stop>>, HashMap<i32, Rc<Stop>>), Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(13, -1, ExpectedType::String),
    ];
    let row_parser = SingleConfigurationRowParser::new(row_configuration);
    let file_parser = FileParser::new("data/BAHNHOF", Box::new(row_parser))?;

    let stops = file_parser
        .iter()
        .map(|(_, mut values)| {
            let id = i32::from(values.remove(0));
            let raw_name = String::from(values.remove(0));

            let parsed_name = super::parse_stop_name(raw_name);

            let name = parsed_name.get(&1).unwrap()[0].clone();
            let long_name = parsed_name.get(&2).map(|x| x[0].clone());
            let abbreviation = parsed_name.get(&3).map(|x| x[0].clone());
            let synonyms = parsed_name.get(&4).cloned();

            Rc::new(Stop::new(id, name, long_name, abbreviation, synonyms))
        })
        .collect();

    let stops_index = create_stops_index(&stops);
    load_lv95_stop_coordinates(&stops_index)?;
    load_wgs84_stop_coordinates(&stops_index)?;

    Ok((stops, stops_index))
}

fn create_stops_index(stops: &Vec<Rc<Stop>>) -> HashMap<i32, Rc<Stop>> {
    stops.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(*item.id(), Rc::clone(item));
        acc
    })
}

pub fn load_lv95_stop_coordinates(
    stops_index: &HashMap<i32, Rc<Stop>>,
) -> Result<(), Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(9, 18, ExpectedType::Float),
        ColumnDefinition::new(20, 29, ExpectedType::Float),
        ColumnDefinition::new(31, 36, ExpectedType::Integer16),
    ];
    let row_parser = SingleConfigurationRowParser::new(row_configuration);
    let file_parser = FileParser::new("data/BFKOORD_LV95", Box::new(row_parser))?;

    for (_, mut values) in file_parser.iter() {
        let stop_id = i32::from(values.remove(0));
        let easting = f64::from(values.remove(0));
        let northing = f64::from(values.remove(0));
        let altitude = i16::from(values.remove(0));

        let coordinate =
            Coordinate::new(CoordinateType::LV95, easting, northing, altitude, stop_id);
        stops_index
            .get(&stop_id)
            .unwrap()
            .set_lv95_coordinate(coordinate);
    }

    Ok(())
}

pub fn load_wgs84_stop_coordinates(
    stops_index: &HashMap<i32, Rc<Stop>>,
) -> Result<(), Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(9, 18, ExpectedType::Float),
        ColumnDefinition::new(20, 29, ExpectedType::Float),
        ColumnDefinition::new(31, 36, ExpectedType::Integer16),
    ];
    let row_parser = SingleConfigurationRowParser::new(row_configuration);
    let file_parser = FileParser::new("data/BFKOORD_WGS", Box::new(row_parser))?;

    for (_, mut values) in file_parser.iter() {
        let stop_id = i32::from(values.remove(0));
        let longitude = f64::from(values.remove(0));
        let latitude = f64::from(values.remove(0));
        let altitude = i16::from(values.remove(0));

        let coordinate = Coordinate::new(
            CoordinateType::WGS84,
            latitude,
            longitude,
            altitude,
            stop_id,
        );
        stops_index
            .get(&stop_id)
            .unwrap()
            .set_wgs84_coordinate(coordinate);
    }

    Ok(())
}
