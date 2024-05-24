// 3 file(s).
// File(s) read by the parser:
// GLEIS, GLEIS_LV95, GLEIS_WGS
// ---
// Note: this parser collects both the Platform and JourneyPlatform resources.
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{
        Coordinate, CoordinateType, Journey, JourneyPlatform, Model, Platform, ResourceIndex, Time,
    },
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, ParsedValue, RowDefinition,
        RowParser,
    },
    storage::SimpleResourceStorage,
    utils::AutoIncrement,
};

pub fn parse(
    journeys_original_primary_index: &ResourceIndex<(i32, String), Journey>,
) -> Result<
    (
        SimpleResourceStorage<JourneyPlatform>,
        SimpleResourceStorage<Platform>,
    ),
    Box<dyn Error>,
> {
    println!("Parsing GLEIS...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a JourneyPlatform instance.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(9, 1, "#", false)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            ColumnDefinition::new(24, 30, ExpectedType::Integer32), // Should be 23-30, but here the # character is ignored.
            ColumnDefinition::new(32, 35, ExpectedType::OptionInteger32),
            ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
        ]),
        // This row is used to create a Platform instance.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(9, 1, "#", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(18, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new("data/GLEIS", row_parser)?;

    let auto_increment = AutoIncrement::new();
    let mut bytes_offset = 0;
    let mut journey_platform_data = Vec::new();
    let mut platforms_original_primary_index = HashMap::new();

    let platforms = parser
        .parse()
        .filter_map(|(id, bytes_read, values)| {
            match id {
                ROW_A => {
                    journey_platform_data.push(values);
                    bytes_offset += bytes_read;
                }
                ROW_B => {
                    let (instance, k) = create_platform(values, &auto_increment);
                    platforms_original_primary_index.insert(k, Rc::clone(&instance));
                    return Some(instance);
                }
                _ => unreachable!(),
            };
            None
        })
        .collect();

    let journey_platform = journey_platform_data
        .into_iter()
        .map(|values| {
            create_journey_platform(
                values,
                journeys_original_primary_index,
                &platforms_original_primary_index,
            )
        })
        .collect();

    println!("Parsing GLEIS_LV95...");
    #[rustfmt::skip]
    load_coordinates_for_platforms(CoordinateType::LV95, bytes_offset, &platforms_original_primary_index)?;
    println!("Parsing GLEIS_WGS84...");
    #[rustfmt::skip]
    load_coordinates_for_platforms(CoordinateType::WGS84, bytes_offset, &platforms_original_primary_index)?;

    Ok((
        SimpleResourceStorage::new(journey_platform),
        SimpleResourceStorage::new(platforms),
    ))
}

fn load_coordinates_for_platforms(
    coordinate_type: CoordinateType,
    bytes_offset: u64,
    platforms_original_primary_index: &HashMap<(i32, i32), Rc<Platform>>,
) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored, as the data has already been retrieved from the GLEIS file.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(18, 1, "G", true)), Vec::new()),
        // This row contains the SLOID.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(18, 3, "I A", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(22, -1, ExpectedType::String),    // This column has not been explicitly defined in the SBB specification.
        ]),
        // This row contains the LV95/WGS84 coordinates.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(18, 1, "K", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(20, 26, ExpectedType::Float),     // This column has not been explicitly defined in the SBB specification.
            ColumnDefinition::new(28, 34, ExpectedType::Float),     // This column has not been explicitly defined in the SBB specification.
        ]),
    ]);
    let filename = match coordinate_type {
        CoordinateType::LV95 => "GLEIS_LV95",
        CoordinateType::WGS84 => "GLEIS_WGS",
    };
    let path = format!("data/{}", filename);
    let parser = FileParser::new_with_bytes_offset(&path, row_parser, bytes_offset)?;

    parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => {}
        ROW_B => set_sloid_of_platform(values, coordinate_type, platforms_original_primary_index),
        ROW_C => {
            set_coordinate_of_platform(values, coordinate_type, platforms_original_primary_index)
        }
        _ => unreachable!(),
    });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn create_journey_platform(
    mut values: Vec<ParsedValue>,
    journeys_original_primary_index: &ResourceIndex<(i32, String), Journey>,
    platforms_original_primary_index: &HashMap<(i32, i32), Rc<Platform>>,
) -> Rc<JourneyPlatform> {
    let stop_id: i32 = values.remove(0).into();
    let journey_id: i32 = values.remove(0).into();
    let administration: String = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let time: Option<i32> = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();

    let journey_id = journeys_original_primary_index
        .get(&(journey_id, administration))
        .unwrap()
        .id();

    let platform_id = platforms_original_primary_index
        .get(&(stop_id, index))
        .unwrap()
        .id();

    let time = time.map(|x| Time::from(x));

    Rc::new(JourneyPlatform::new(
        journey_id,
        platform_id,
        time,
        bit_field_id,
    ))
}

fn create_platform(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
) -> (Rc<Platform>, (i32, i32)) {
    let stop_id: i32 = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let platform_data: String = values.remove(0).into();

    let (code, sectors) = parse_platform_data(platform_data);

    let instance = Rc::new(Platform::new(auto_increment.next(), code, sectors, stop_id));
    (instance, (stop_id, index))
}

fn set_sloid_of_platform(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    platforms_original_primary_index: &HashMap<(i32, i32), Rc<Platform>>,
) {
    // The SLOID is processed only when loading LV95 coordinates.
    if coordinate_type == CoordinateType::LV95 {
        let stop_id: i32 = values.remove(0).into();
        let index: i32 = values.remove(0).into();
        let sloid: String = values.remove(0).into();

        platforms_original_primary_index
            .get(&(stop_id, index))
            .unwrap()
            .set_sloid(sloid);
    }
}

fn set_coordinate_of_platform(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    platforms_original_primary_index: &HashMap<(i32, i32), Rc<Platform>>,
) {
    let stop_id: i32 = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let mut xy1: f64 = values.remove(0).into();
    let mut xy2: f64 = values.remove(0).into();
    // Altitude is not provided for platforms.
    let altitude = 0;

    if coordinate_type == CoordinateType::WGS84 {
        // WGS84 coordinates are stored in reverse order for some unknown reason.
        (xy1, xy2) = (xy2, xy1);
    }

    let coordinate = Coordinate::new(coordinate_type, xy1, xy2, altitude);
    let platform = platforms_original_primary_index
        .get(&(stop_id, index))
        .unwrap();

    match coordinate_type {
        CoordinateType::LV95 => platform.set_lv95_coordinate(coordinate),
        CoordinateType::WGS84 => platform.set_wgs84_coordinate(coordinate),
    }
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_platform_data(mut platform_data: String) -> (String, Option<String>) {
    platform_data = format!("{} ", platform_data);
    let data = platform_data.split("' ").filter(|&s| !s.is_empty()).fold(
        HashMap::new(),
        |mut acc, item| {
            let parts: Vec<&str> = item.split(" '").collect();
            acc.insert(parts[0], parts[1]);
            acc
        },
    );

    // There should always be a G entry.
    let code = data.get("G").unwrap().to_string();
    let sectors = data.get("A").map(|s| s.to_string());

    (code, sectors)
}
