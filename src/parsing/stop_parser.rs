// 8.5 file(s).
// File(s) read by the parser:
// BAHNHOF, BFKOORD_LV95, BFKOORD_WGS, BFPRIOS, KMINFO, UMSTEIGB, METABHF, BHFART_60
// ---
// Files not used by the parser:
// BHFART
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Coordinate, CoordinateType, Model, PrimaryIndex, Stop},
    parsing::{AdvancedRowMatcher, FastRowMatcher}, storage::SimpleDataStorage,
};

use super::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser};

pub fn parse() -> Result<SimpleDataStorage<Stop>, Box<dyn Error>> {
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

    let primary_index = Stop::create_primary_index(&rows);

    println!("Parsing BFKOORD_LV95...");
    load_coordinates(CoordinateType::LV95, &primary_index)?;
    println!("Parsing BFKOORD_WGS...");
    load_coordinates(CoordinateType::WGS84, &primary_index)?;
    println!("Parsing BFPRIOS...");
    load_changing_priorities(&primary_index)?;
    println!("Parsing KMINFO...");
    load_changing_flags(&primary_index)?;
    println!("Parsing UMSTEIGB...");
    load_changing_times(&primary_index)?;
    println!("Parsing METABHF 2/2...");
    load_connections(&primary_index)?;
    println!("Parsing BHFART_60...");
    load_descriptions(&primary_index)?;

    Ok(SimpleDataStorage::new(rows))
}

fn load_coordinates(
    coordinate_type: CoordinateType,
    primary_index: &PrimaryIndex<Stop>,
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

fn load_changing_priorities(primary_index: &PrimaryIndex<Stop>) -> Result<(), Box<dyn Error>> {
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
    // TODO: default value should be 8.

    Ok(())
}

fn load_changing_flags(primary_index: &PrimaryIndex<Stop>) -> Result<(), Box<dyn Error>> {
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

fn load_changing_times(primary_index: &PrimaryIndex<Stop>) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing time.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 10, ExpectedType::Integer16),
            ColumnDefinition::new(12, 13, ExpectedType::Integer16),
        ]),
    ]);
    let file_path = "data/UMSTEIGB";
    let file_parser = FileParser::new(&file_path, row_parser)?;

    file_parser
        .parse()
        .for_each(|(_, _, values)| set_changing_time(values, primary_index));

    Ok(())
}

fn load_connections(primary_index: &PrimaryIndex<Stop>) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored.
        RowDefinition::new(ROW_A, Box::new(AdvancedRowMatcher::new("[0-9]{7} [0-9]{7} [0-9]{3}")?), Vec::new()),
        // This row is ignored.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 1, "*", true)), Vec::new()),
        //
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(8, 1, ":", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(11, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/METABHF", row_parser)?;

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_A | ROW_B => return,
        ROW_C => set_connections(values, primary_index),
        _ => unreachable!(),
    });

    Ok(())
}

fn load_descriptions(primary_index: &PrimaryIndex<Stop>) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(1, 1, "%", true)), Vec::new()),
        // Restrictions.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(9, 1, "B", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(11, 12, ExpectedType::Integer16),
        ]),
        // SLOID.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(11, 1, "A", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ]),
        // Boarding areas.
        RowDefinition::new(ROW_D, Box::new(FastRowMatcher::new(11, 1, "a", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/BHFART_60", row_parser)?;

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => return,
        ROW_B => set_restrictions(values, primary_index),
        ROW_C => set_sloid(values, primary_index),
        ROW_D => add_boarding_area(values, primary_index),
        _ => unreachable!(),
    });

    Ok(())
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
    primary_index: &PrimaryIndex<Stop>,
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

fn set_changing_priority(mut values: Vec<ParsedValue>, primary_index: &PrimaryIndex<Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let changing_priority: i16 = values.remove(0).into();

    let stop = primary_index.get(&stop_id).unwrap();
    stop.set_changing_priority(changing_priority);
}

fn set_changing_flag(mut values: Vec<ParsedValue>, primary_index: &PrimaryIndex<Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let changing_flag: i16 = values.remove(0).into();

    let stop = primary_index.get(&stop_id).unwrap();
    stop.set_changing_flag(changing_flag);
}

fn set_changing_time(mut values: Vec<ParsedValue>, primary_index: &PrimaryIndex<Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let changing_time_inter_city: i16 = values.remove(0).into();
    let changing_time_other: i16 = values.remove(0).into();

    if stop_id == 9999999 {
        // The first row of the file has the stop ID number 9999999. It contains the default values for all stops.
        for (_, stop) in primary_index.iter() {
            stop.set_changing_time_inter_city(changing_time_inter_city);
            stop.set_changing_time_other(changing_time_other);
        }
    } else {
        let stop = primary_index.get(&stop_id).unwrap();
        stop.set_changing_time_inter_city(changing_time_inter_city);
        stop.set_changing_time_other(changing_time_other);
    }
}

fn set_connections(mut values: Vec<ParsedValue>, primary_index: &PrimaryIndex<Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let connections: String = values.remove(0).into();
    let connections = parse_connections(connections);

    let stop = primary_index.get(&stop_id).unwrap();
    stop.set_connections(connections);
}

fn set_restrictions(mut values: Vec<ParsedValue>, primary_index: &PrimaryIndex<Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let restrictions: i16 = values.remove(0).into();

    let stop = primary_index.get(&stop_id).unwrap();
    stop.set_restrictions(restrictions);
}

fn set_sloid(mut values: Vec<ParsedValue>, primary_index: &PrimaryIndex<Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let sloid: String = values.remove(0).into();

    let stop = primary_index.get(&stop_id).unwrap();
    stop.set_sloid(sloid);
}

fn add_boarding_area(mut values: Vec<ParsedValue>, primary_index: &PrimaryIndex<Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let sloid: String = values.remove(0).into();

    let stop = primary_index.get(&stop_id).unwrap();
    stop.add_boarding_area(sloid);
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

fn parse_connections(raw_connections: String) -> Vec<i32> {
    raw_connections
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
