// 8.5 file(s).
// File(s) read by the parser:
// BAHNHOF, BFKOORD_LV95, BFKOORD_WGS, BFPRIOS, KMINFO, UMSTEIGB, METABHF, BHFART_60
// ---
// Files not used by the parser:
// BHFART
use std::{collections::HashMap, error::Error};

use crate::{
    models::{Coordinate, CoordinateType, Model, Stop},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        ParsedValue, RowDefinition, RowParser,
    },
    storage::SimpleResourceStorage,
};

pub fn parse() -> Result<SimpleResourceStorage<Stop>, Box<dyn Error>> {
    println!("Parsing BAHNHOF...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Stop instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String), // Should be 13-62, but some entries go beyond column 62.
        ]),
    ]);
    let parser = FileParser::new("data/BAHNHOF", row_parser)?;

    let data = parser
        .parse()
        .map(|(_, _, values)| create_instance(values))
        .collect();
    let mut data = Stop::vec_to_map(data);

    println!("Parsing BFKOORD_LV95...");
    load_coordinates(CoordinateType::LV95, &mut data)?;
    println!("Parsing BFKOORD_WGS...");
    load_coordinates(CoordinateType::WGS84, &mut data)?;
    println!("Parsing BFPRIOS...");
    load_transfer_priorities(&mut data)?;
    println!("Parsing KMINFO...");
    load_transfer_flags(&mut data)?;
    println!("Parsing UMSTEIGB...");
    load_transfer_times(&mut data)?;
    println!("Parsing METABHF 1/2...");
    load_connections(&mut data)?;
    println!("Parsing BHFART_60...");
    load_descriptions(&mut data)?;

    Ok(SimpleResourceStorage::new(data))
}

fn load_coordinates(
    coordinate_type: CoordinateType,
    data: &mut HashMap<i32, Stop>,
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
    let path = format!("data/{}", filename);
    let parser = FileParser::new(&path, row_parser)?;

    parser
        .parse()
        .for_each(|(_, _, values)| set_coordinate(values, coordinate_type, data));

    Ok(())
}

fn load_transfer_priorities(data: &mut HashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing priority.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 10, ExpectedType::Integer16),
        ]),
    ]);
    let path = "data/BFPRIOS";
    let parser = FileParser::new(&path, row_parser)?;

    parser
        .parse()
        .for_each(|(_, _, values)| set_transfer_priority(values, data));

    Ok(())
}

fn load_transfer_flags(data: &mut HashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing flag.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 13, ExpectedType::Integer16),
        ]),
    ]);
    let parser = FileParser::new("data/KMINFO", row_parser)?;

    parser
        .parse()
        .for_each(|(_, _, values)| set_transfer_flag(values, data));

    Ok(())
}

fn load_transfer_times(data: &mut HashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing time.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 10, ExpectedType::Integer16),
            ColumnDefinition::new(12, 13, ExpectedType::Integer16),
        ]),
    ]);
    let parser = FileParser::new("data/UMSTEIGB", row_parser)?;

    parser
        .parse()
        .for_each(|(_, _, values)| set_transfer_time(values, data));

    Ok(())
}

fn load_connections(data: &mut HashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored.
        RowDefinition::new(ROW_A, Box::new(AdvancedRowMatcher::new(r"[0-9]{7} [0-9]{7} [0-9]{3}")?), Vec::new()),
        // This row is ignored.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 1, "*", true)), Vec::new()),
        // This row contains the connections to nearby stops.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(8, 1, ":", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(11, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new("data/METABHF", row_parser)?;

    parser.parse().for_each(|(id, _, values)| match id {
        ROW_A | ROW_B => {}
        ROW_C => set_connections(values, data),
        _ => unreachable!(),
    });

    Ok(())
}

fn load_descriptions(data: &mut HashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(1, 1, "%", true)), Vec::new()),
        // This row contains the restrictions.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(9, 1, "B", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(11, 12, ExpectedType::Integer16),
        ]),
        // This row contains the SLOID.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(11, 1, "A", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ]),
        // This row contains the boarding areas.
        RowDefinition::new(ROW_D, Box::new(FastRowMatcher::new(11, 1, "a", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new("data/BHFART_60", row_parser)?;

    parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => {}
        ROW_B => set_restrictions(values, data),
        ROW_C => set_sloid(values, data),
        ROW_D => add_boarding_area(values, data),
        _ => unreachable!(),
    });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Stop {
    let id: i32 = values.remove(0).into();
    let designations: String = values.remove(0).into();

    let (name, long_name, abbreviation, synonyms) = parse_designations(designations);

    Stop::new(id, name, long_name, abbreviation, synonyms)
}

fn set_coordinate(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    data: &mut HashMap<i32, Stop>,
) {
    let stop_id: i32 = values.remove(0).into();
    let mut xy1: f64 = values.remove(0).into();
    let mut xy2: f64 = values.remove(0).into();
    let altitude: i16 = values.remove(0).into();

    if coordinate_type == CoordinateType::WGS84 {
        // WGS84 coordinates are stored in reverse order for some unknown reason.
        (xy1, xy2) = (xy2, xy1);
    }

    let stop = data.get_mut(&stop_id).unwrap();
    let coordinate = Coordinate::new(coordinate_type, xy1, xy2, altitude);

    match coordinate_type {
        CoordinateType::LV95 => stop.set_lv95_coordinate(coordinate),
        CoordinateType::WGS84 => stop.set_wgs84_coordinate(coordinate),
    }
}

fn set_transfer_priority(mut values: Vec<ParsedValue>, data: &mut HashMap<i32, Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let transfer_priority: i16 = values.remove(0).into();

    let stop = data.get_mut(&stop_id).unwrap();
    stop.set_transfer_priority(transfer_priority);
}

fn set_transfer_flag(mut values: Vec<ParsedValue>, data: &mut HashMap<i32, Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let transfer_flag: i16 = values.remove(0).into();

    let stop = data.get_mut(&stop_id).unwrap();
    stop.set_transfer_flag(transfer_flag);
}

fn set_transfer_time(mut values: Vec<ParsedValue>, data: &mut HashMap<i32, Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let transfer_time_inter_city: i16 = values.remove(0).into();
    let transfer_time_other: i16 = values.remove(0).into();

    if stop_id == 9999999 {
        // The first row of the file has the stop ID number 9999999. It contains the default values for all stops.
        for stop in data.values_mut() {
            stop.set_transfer_time_inter_city(transfer_time_inter_city);
            stop.set_transfer_time_other(transfer_time_other);
        }
    } else {
        let stop = data.get_mut(&stop_id).unwrap();
        stop.set_transfer_time_inter_city(transfer_time_inter_city);
        stop.set_transfer_time_other(transfer_time_other);
    }
}

fn set_connections(mut values: Vec<ParsedValue>, data: &mut HashMap<i32, Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let connections: String = values.remove(0).into();

    let connections = parse_connections(connections);

    let stop = data.get_mut(&stop_id).unwrap();
    stop.set_connections(connections);
}

fn set_restrictions(mut values: Vec<ParsedValue>, data: &mut HashMap<i32, Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let restrictions: i16 = values.remove(0).into();

    let stop = data.get_mut(&stop_id).unwrap();
    stop.set_restrictions(restrictions);
}

fn set_sloid(mut values: Vec<ParsedValue>, data: &mut HashMap<i32, Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let sloid: String = values.remove(0).into();

    let stop = data.get_mut(&stop_id).unwrap();
    stop.set_sloid(sloid);
}

fn add_boarding_area(mut values: Vec<ParsedValue>, data: &mut HashMap<i32, Stop>) {
    let stop_id: i32 = values.remove(0).into();
    let sloid: String = values.remove(0).into();

    let stop = data.get_mut(&stop_id).unwrap();
    stop.add_boarding_area(sloid);
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_designations(
    designations: String,
) -> (String, Option<String>, Option<String>, Option<Vec<String>>) {
    let designations: HashMap<i32, Vec<String>> = designations
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

    let name = designations.get(&1).unwrap()[0].clone();
    let long_name = designations.get(&2).map(|x| x[0].clone());
    let abbreviation = designations.get(&3).map(|x| x[0].clone());
    let synonyms = designations.get(&4).cloned();

    (name, long_name, abbreviation, synonyms)
}

fn parse_connections(connections: String) -> Vec<i32> {
    connections
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}
