// 5 file(s).
// File(s) read by the parser:
// BAHNHOF => Format matches the standard.
// BFKOORD_LV95 => Format matches the standard.
// BFKOORD_WGS => Format matches the standard.
// BFPRIOS => Format matches the standard.
// KMINFO =>
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Coordinate, CoordinateType, Stop, StopCollection, StopPrimaryIndex},
    storage::StopData,
};

use super::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser};

pub fn parse() -> Result<StopData, Box<dyn Error>> {
    println!("Parsing BAHNHOF...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Stop instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),  // Should be 13-62, but some entries go beyond column 62.
        ]),
    ]);
    let file_parser = FileParser::new("data/BAHNHOF", row_parser)?;

    let rows = file_parser
        .parse()
        .map(|(_, _, values)| create_instance(values))
        .collect();

    let primary_index = create_primary_index(&rows);

    println!("Parsing BFKOORD_LV95...");
    load_coordinates(CoordinateType::LV95, &primary_index)?;
    println!("Parsing BFKOORD_WGS...");
    load_coordinates(CoordinateType::WGS84, &primary_index)?;
    println!("Parsing BFPRIOS...");
    load_changing_priorities(&primary_index)?;
    println!("Parsing KMINFO...");
    load_changing_flags(&primary_index)?;

    Ok(StopData::new(rows, primary_index))
}

fn load_coordinates(
    coordinate_type: CoordinateType,
    primary_index: &StopPrimaryIndex,
) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the LV95/WGS84 coordinates.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 18, ExpectedType::Float),
            ColumnDefinition::new(20, 29, ExpectedType::Float),
            ColumnDefinition::new(31, 36, ExpectedType::Integer16),
        ]),
    ]);
    let filename = match coordinate_type {
        CoordinateType::LV95 => "BFKOORD_LV95",
        CoordinateType::WGS84 => "BFKOORD_WGS",
    };
    let file_path = format!("data/{}", filename);
    let file_parser = FileParser::new(&file_path, row_parser)?;

    file_parser
        .parse()
        .for_each(|(_, _, values)| set_coordinate(values, coordinate_type, primary_index));

    Ok(())
}

fn load_changing_priorities(primary_index: &StopPrimaryIndex) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing priority.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 10, ExpectedType::Integer16),
        ]),
    ]);
    let file_path = "data/BFPRIOS";
    let file_parser = FileParser::new(&file_path, row_parser)?;

    file_parser
        .parse()
        .for_each(|(_, _, values)| set_changing_priority(values, primary_index));

    Ok(())
}

fn load_changing_flags(primary_index: &StopPrimaryIndex) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing flag.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 13, ExpectedType::Integer16),
        ]),
    ]);
    let file_path = "data/KMINFO";
    let file_parser = FileParser::new(&file_path, row_parser)?;

    file_parser
        .parse()
        .for_each(|(_, _, values)| set_changing_flag(values, primary_index));

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_primary_index(rows: &StopCollection) -> StopPrimaryIndex {
    rows.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<Stop> {
    let id: i32 = values.remove(0).into();
    let raw_name: String = values.remove(0).into();

    let (name, long_name, abbreviation, synonyms) = parse_name(raw_name);

    Rc::new(Stop::new(id, name, long_name, abbreviation, synonyms))
}

fn set_coordinate(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    primary_index: &StopPrimaryIndex,
) {
    let stop_id: i32 = values.remove(0).into();
    let mut xy1: f64 = values.remove(0).into();
    let mut xy2: f64 = values.remove(0).into();
    let altitude: i16 = values.remove(0).into();

    if coordinate_type == CoordinateType::WGS84 {
        // WGS84 coordinates are stored in reverse order for some unknown reason.
        (xy1, xy2) = (xy2, xy1);
    }

    let stop = primary_index.get(&stop_id).unwrap();
    let coordinate = Coordinate::new(coordinate_type, xy1, xy2, altitude);

    match coordinate_type {
        CoordinateType::LV95 => stop.set_lv95_coordinate(coordinate),
        CoordinateType::WGS84 => stop.set_wgs84_coordinate(coordinate),
    }
}

fn set_changing_priority(mut values: Vec<ParsedValue>, primary_index: &StopPrimaryIndex) {
    let stop_id: i32 = values.remove(0).into();
    let changing_priority: i16 = values.remove(0).into();

    let stop = primary_index.get(&stop_id).unwrap();
    stop.set_changing_priority(changing_priority);
}

fn set_changing_flag(mut values: Vec<ParsedValue>, primary_index: &StopPrimaryIndex) {
    let stop_id: i32 = values.remove(0).into();
    let changing_flag: i16 = values.remove(0).into();

    let stop = primary_index.get(&stop_id).unwrap();
    stop.set_changing_flag(changing_flag);
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_name(raw_name: String) -> (String, Option<String>, Option<String>, Option<Vec<String>>) {
    let data: HashMap<i32, Vec<String>> = raw_name
        .split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| {
            let s = s.replace('$', "");
            let mut parts = s.split('<');

            let v = parts.next().unwrap().to_string();
            let k = parts.next().unwrap().parse::<i32>().unwrap();

            (k, v)
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k).or_insert(Vec::new()).push(v);
            acc
        });

    let name = data.get(&1).unwrap()[0].clone();
    let long_name = data.get(&2).map(|x| x[0].clone());
    let abbreviation = data.get(&3).map(|x| x[0].clone());
    let synonyms = data.get(&4).cloned();

    (name, long_name, abbreviation, synonyms)
}
