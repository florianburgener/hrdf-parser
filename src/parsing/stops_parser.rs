// BAHNHOF, BFKOORD_LV95 (BF = BAHNHOF), BFKOORD_WGS (BF = BAHNHOF)
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::models::{Coordinate, CoordinateType, Stop};

use super::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser};

pub fn load_stops() -> Result<(Vec<Rc<Stop>>, HashMap<i32, Rc<Stop>>), Box<dyn Error>> {
    println!("Parsing BAHNHOF...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32), // Complies with the standard.
            ColumnDefinition::new(13, -1, ExpectedType::String),  // Does not comply with the standard. Should be 13-62, but some entries go beyond column 62.
        ]),
    ]);
    let file_parser = FileParser::new("data/BAHNHOF", row_parser)?;

    let stops = file_parser
        .parse()
        .map(|(_, _, mut values)| {
            let id: i32 = values.remove(0).into();
            let raw_name: String = values.remove(0).into();

            let parsed_name = parse_stop_name(raw_name);

            let name = parsed_name.get(&1).unwrap()[0].clone();
            let long_name = parsed_name.get(&2).map(|x| x[0].clone());
            let abbreviation = parsed_name.get(&3).map(|x| x[0].clone());
            let synonyms = parsed_name.get(&4).cloned();

            Rc::new(Stop::new(id, name, long_name, abbreviation, synonyms))
        })
        .collect();

    let stops_index = create_stops_index(&stops);
    println!("Parsing BFKOORD_LV95...");
    load_coordinates(CoordinateType::LV95, &stops_index)?;
    println!("Parsing BFKOORD_WGS...");
    load_coordinates(CoordinateType::WGS84, &stops_index)?;

    Ok((stops, stops_index))
}

fn create_stops_index(stops: &Vec<Rc<Stop>>) -> HashMap<i32, Rc<Stop>> {
    stops.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id(), Rc::clone(item));
        acc
    })
}

fn load_coordinates(
    coordinate_type: CoordinateType,
    stops_index: &HashMap<i32, Rc<Stop>>,
) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),   // Complies with the standard.
            ColumnDefinition::new(9, 18, ExpectedType::Float),      // Complies with the standard.
            ColumnDefinition::new(20, 29, ExpectedType::Float),     // Complies with the standard.
            ColumnDefinition::new(31, 36, ExpectedType::Integer16), // Complies with the standard.
        ]),
    ]);
    let filename = match coordinate_type {
            CoordinateType::LV95 => "BFKOORD_LV95",
            CoordinateType::WGS84 => "BFKOORD_WGS",
        };
    let file_path = format!("data/{}", filename);
    let file_parser = FileParser::new(&file_path, row_parser)?;

    for (_, _, mut values) in file_parser.parse() {
        let stop_id: i32 = values.remove(0).into();
        let mut xy1: f64 = values.remove(0).into();
        let mut xy2: f64 = values.remove(0).into();
        let altitude: i16 = values.remove(0).into();

        if coordinate_type == CoordinateType::WGS84 {
            // WGS84 coordinates are stored in reverse order for some unknown reason.
            (xy1, xy2) = (xy2, xy1);
        }

        let stop = stops_index.get(&stop_id).unwrap();
        let coordinate = Coordinate::new(coordinate_type, xy1, xy2, altitude);

        match coordinate_type {
            CoordinateType::LV95 => stop.set_lv95_coordinate(coordinate),
            CoordinateType::WGS84 => stop.set_wgs84_coordinate(coordinate),
        }
    }

    Ok(())
}

fn parse_stop_name(name: String) -> HashMap<i32, Vec<String>> {
    let parsed_name: HashMap<i32, Vec<String>> = name
        .split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| s.replace('$', ""))
        .map(|s| {
            let mut parts = s.split('<');

            let value = parts.next().unwrap().to_string();
            let key = parts.next().unwrap().parse::<i32>().unwrap();

            (key, value)
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert(Vec::new()).push(value);
            acc
        });
    parsed_name
}
