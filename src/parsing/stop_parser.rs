// 8.5 file(s).
// File(s) read by the parser:
// BAHNHOF, BFKOORD_LV95, BFKOORD_WGS, BFPRIOS, KMINFO, UMSTEIGB, METABHF, BHFART_60
// ---
// Files not used by the parser:
// BHFART
use std::{error::Error, vec};

use rustc_hash::FxHashMap;

use crate::{
    models::{CoordinateSystem, Coordinates, Model, Stop, Version},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        ParsedValue, RowDefinition, RowParser,
    },
    storage::ResourceStorage,
};

pub fn parse(
    version: Version,
    path: &str,
) -> Result<(ResourceStorage<Stop>, (i16, i16)), Box<dyn Error>> {
    println!("Parsing BAHNHOF...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Stop instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String), // Should be 13-62, but some entries go beyond column 62.
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/BAHNHOF"), row_parser)?;

    let data = parser
        .parse()
        .map(|x| x.map(|(_, _, values)| create_instance(values)))
        .collect::<Result<Vec<_>, _>>()?;
    let mut data = Stop::vec_to_map(data);

    println!("Parsing BFKOORD_LV95...");
    load_coordinates(version, path, CoordinateSystem::LV95, &mut data)?;
    println!("Parsing BFKOORD_WGS...");
    load_coordinates(version, path, CoordinateSystem::WGS84, &mut data)?;
    println!("Parsing BFPRIOS...");
    load_exchange_priorities(path, &mut data)?;
    println!("Parsing KMINFO...");
    load_exchange_flags(path, &mut data)?;
    println!("Parsing UMSTEIGB...");
    let default_exchange_time = load_exchange_times(path, &mut data)?;
    println!("Parsing METABHF 1/2...");
    load_connections(path, &mut data)?;
    println!("Parsing BHFART_60...");
    load_descriptions(path, &mut data)?;

    Ok((ResourceStorage::new(data), default_exchange_time))
}

fn load_coordinates(
    version: Version,
    path: &str,
    coordinate_system: CoordinateSystem,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the LV95/WGS84 coordinates.
        RowDefinition::from(
            match version {
                Version::V_5_40_41_2_0_4 => vec![
                    ColumnDefinition::new(1, 7, ExpectedType::Integer32),
                    ColumnDefinition::new(9, 18, ExpectedType::Float),
                    ColumnDefinition::new(20, 29, ExpectedType::Float),
                    ColumnDefinition::new(31, 36, ExpectedType::Integer16),
                ],
                Version::V_5_40_41_2_0_5 => vec![
                    ColumnDefinition::new(1, 7, ExpectedType::Integer32),
                    ColumnDefinition::new(9, 19, ExpectedType::Float),
                    ColumnDefinition::new(21, 31, ExpectedType::Float),
                    ColumnDefinition::new(33, 39, ExpectedType::Integer16),
                ],
            }
        ),
    ]);
    let filename = match coordinate_system {
        CoordinateSystem::LV95 => "BFKOORD_LV95",
        CoordinateSystem::WGS84 => "BFKOORD_WGS",
    };
    let parser = FileParser::new(&format!("{path}/{filename}"), row_parser)?;

    parser.parse().try_for_each(|x| {
        let (_, _, values) = x?;
        set_coordinates(values, coordinate_system, data)?;
        Ok(())
    })
}

fn load_exchange_priorities(
    path: &str,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing priority.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 10, ExpectedType::Integer16),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/BFPRIOS"), row_parser)?;

    parser.parse().try_for_each(|x| {
        let (_, _, values) = x?;
        set_exchange_priority(values, data)?;
        Ok(())
    })
}

fn load_exchange_flags(path: &str, data: &mut FxHashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing flag.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 13, ExpectedType::Integer16),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/KMINFO"), row_parser)?;

    parser.parse().try_for_each(|x| {
        let (_, _, values) = x?;
        set_exchange_flag(values, data)?;
        Ok(())
    })
}

fn load_exchange_times(
    path: &str,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(i16, i16), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the changing time.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 10, ExpectedType::Integer16),
            ColumnDefinition::new(12, 13, ExpectedType::Integer16),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/UMSTEIGB"), row_parser)?;

    let mut default_exchange_time = (0, 0);

    parser.parse().try_for_each(|x| {
        let (_, _, values) = x?;
        if let Some(x) = set_exchange_time(values, data)? {
            default_exchange_time = x;
        }
        Ok::<(), Box<dyn Error>>(())
    })?;

    Ok(default_exchange_time)
}

fn load_connections(path: &str, data: &mut FxHashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
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
    let parser = FileParser::new(&format!("{path}/METABHF"), row_parser)?;

    parser.parse().try_for_each(|x| {
        let (id, _, values) = x?;
        match id {
            ROW_A | ROW_B => {}
            ROW_C => set_connections(values, data)?,
            _ => unreachable!(),
        }
        Ok(())
    })
}

fn load_descriptions(path: &str, data: &mut FxHashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
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
    let parser = FileParser::new(&format!("{path}/BHFART_60"), row_parser)?;

    parser.parse().try_for_each(|x| {
        let (id, _, values) = x?;
        match id {
            ROW_A => {}
            ROW_B => set_restrictions(values, data)?,
            ROW_C => set_sloid(values, data)?,
            ROW_D => add_boarding_area(values, data)?,
            _ => unreachable!(),
        }
        Ok(())
    })
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

fn set_coordinates(
    mut values: Vec<ParsedValue>,
    coordinate_system: CoordinateSystem,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let mut xy1: f64 = values.remove(0).into();
    let mut xy2: f64 = values.remove(0).into();
    // Altitude is not stored, as it is not provided for 95% of stops.
    let _altitude: i16 = values.remove(0).into();

    if coordinate_system == CoordinateSystem::WGS84 {
        // WGS84 coordinates are stored in reverse order for some unknown reason.
        (xy1, xy2) = (xy2, xy1);
    }

    let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
    let coordinate = Coordinates::new(coordinate_system, xy1, xy2);

    match coordinate_system {
        CoordinateSystem::LV95 => stop.set_lv95_coordinates(coordinate),
        CoordinateSystem::WGS84 => stop.set_wgs84_coordinates(coordinate),
    }

    Ok(())
}

fn set_exchange_priority(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let exchange_priority: i16 = values.remove(0).into();

    let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
    stop.set_exchange_priority(exchange_priority);

    Ok(())
}

fn set_exchange_flag(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let exchange_flag: i16 = values.remove(0).into();

    let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
    stop.set_exchange_flag(exchange_flag);

    Ok(())
}

fn set_exchange_time(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<Option<(i16, i16)>, Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let exchange_time_inter_city: i16 = values.remove(0).into();
    let exchange_time_other: i16 = values.remove(0).into();

    let exchange_time = Some((exchange_time_inter_city, exchange_time_other));

    if stop_id == 9999999 {
        // The first row of the file has the stop ID number 9999999.
        // It contains default exchange times to be used when a stop has no specific exchange time.
        Ok(exchange_time)
    } else {
        let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
        stop.set_exchange_time(exchange_time);
        Ok(None)
    }
}

fn set_connections(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let connections: String = values.remove(0).into();

    let connections = parse_connections(connections);

    let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
    stop.set_connections(connections);

    Ok(())
}

fn set_restrictions(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let restrictions: i16 = values.remove(0).into();

    let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
    stop.set_restrictions(restrictions);

    Ok(())
}

fn set_sloid(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let sloid: String = values.remove(0).into();

    let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
    stop.set_sloid(sloid);

    Ok(())
}

fn add_boarding_area(
    mut values: Vec<ParsedValue>,
    data: &mut FxHashMap<i32, Stop>,
) -> Result<(), Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let sloid: String = values.remove(0).into();

    let stop = data.get_mut(&stop_id).ok_or("Unknown ID")?;
    stop.add_boarding_area(sloid);

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_designations(
    designations: String,
) -> (String, Option<String>, Option<String>, Option<Vec<String>>) {
    let designations: FxHashMap<i32, Vec<String>> = designations
        .split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| {
            let s = s.replace('$', "");
            let mut parts = s.split('<');

            let v = parts.next().unwrap().to_string();
            let k = parts.next().unwrap().parse::<i32>().unwrap();

            (k, v)
        })
        .fold(FxHashMap::default(), |mut acc, (k, v)| {
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
