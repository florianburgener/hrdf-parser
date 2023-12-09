// GLEIS, GLEIS_LV95, GLEIS_WGS
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Coordinate, CoordinateType, JourneyPlatform, Platform},
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, RowDefinition, RowParser,
    },
};

use super::ParsedValue;

pub fn load_journey_platform_and_platforms() -> Result<
    (
        Vec<Rc<JourneyPlatform>>,
        HashMap<(i32, i64), Vec<Rc<JourneyPlatform>>>,
        Vec<Rc<Platform>>,
        HashMap<i64, Rc<Platform>>,
    ),
    Box<dyn Error>,
> {
    println!("Parsing GLEIS...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(9, 1, "#", false)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),         // Complies with the standard.
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),        // Complies with the standard.
            ColumnDefinition::new(16, 21, ExpectedType::String),          // Complies with the standard.
            ColumnDefinition::new(24, 30, ExpectedType::Integer32),       // Does not comply with the standard. Should be 23-30, but here the # character is ignored.
            ColumnDefinition::new(32, 35, ExpectedType::OptionInteger16), // Complies with the standard.
            ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32), // Complies with the standard.
        ]),
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(9, 1, "#", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),   // Complies with the standard.
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Does not comply with the standard. Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(18, -1, ExpectedType::String),    // Complies with the standard.
        ]),
    ]);
    let file_parser = FileParser::new("data/GLEIS", row_parser)?;

    let mut journey_platform = Vec::new();
    let mut platforms = Vec::new();
    let mut bytes_offset = 0;

    file_parser
        .parse()
        .for_each(|(id, bytes_read, values)| match id {
            ROW_A => {
                bytes_offset += bytes_read;
                journey_platform.push(create_journey_platform(values))
            }
            ROW_B => platforms.push(create_platform(values)),
            _ => unreachable!(),
        });

    let journey_platform_primary_index = create_journey_platform_primary_index(&journey_platform);
    let platforms_primary_index = create_platforms_primary_index(&platforms);

    println!("Parsing GLEIS_LV95...");
    load_coordinates_for_platforms(CoordinateType::LV95, bytes_offset, &platforms_primary_index)?;
    println!("Parsing GLEIS_WGS84...");
    load_coordinates_for_platforms(
        CoordinateType::WGS84,
        bytes_offset,
        &platforms_primary_index,
    )?;

    Ok((
        journey_platform,
        journey_platform_primary_index,
        platforms,
        platforms_primary_index,
    ))
}

fn load_coordinates_for_platforms(
    coordinate_type: CoordinateType,
    bytes_offset: u64,
    platforms_primary_index: &HashMap<i64, Rc<Platform>>,
) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(18, 1, "G", true)), Vec::new()),
        // This row contains the SLOID.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(18, 3, "I A", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),   // Complies with the standard.
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Does not comply with the standard. Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(22, -1, ExpectedType::String),    // Complies with the standard. Please note that the type and columns are not explicitly described in the SBB specification.
        ]),
        // This row contains the LV95/WGS84 coordinates.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(18, 1, "K", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),   // Complies with the standard.
            ColumnDefinition::new(10, 16, ExpectedType::Integer32), // Does not comply with the standard. Should be 9-16, but here the # character is ignored.
            ColumnDefinition::new(20, 26, ExpectedType::Float),     // Complies with the standard. Please note that the type and columns are not explicitly described in the SBB specification.
            ColumnDefinition::new(28, 34, ExpectedType::Float),     // Complies with the standard. Please note that the type and columns are not explicitly described in the SBB specification.
        ]),
    ]);
    let filename = match coordinate_type {
        CoordinateType::LV95 => "GLEIS_LV95",
        CoordinateType::WGS84 => "GLEIS_WGS",
    };
    let file_path = format!("data/{}", filename);
    let file_parser = FileParser::new_with_bytes_offset(&file_path, row_parser, bytes_offset)?;

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => return,
        ROW_B => set_sloid_of_platform(values, coordinate_type, platforms_primary_index),
        ROW_C => set_coordinate_of_platform(values, coordinate_type, platforms_primary_index),
        _ => unreachable!(),
    });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_journey_platform_primary_index(
    journey_platform: &Vec<Rc<JourneyPlatform>>,
) -> HashMap<(i32, i64), Vec<Rc<JourneyPlatform>>> {
    journey_platform
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.entry((item.journey_id(), item.platform_id()))
                .or_insert(Vec::new())
                .push(Rc::clone(item));
            acc
        })
}

fn create_platforms_primary_index(platforms: &Vec<Rc<Platform>>) -> HashMap<i64, Rc<Platform>> {
    platforms.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_platform_data(platform_data: String) -> (String, Option<String>) {
    let cleaned_platform_data = platform_data.trim().to_string() + " ";
    let parsed_values = cleaned_platform_data
        .split("' ")
        .fold(HashMap::new(), |mut acc, item| {
            if item.is_empty() {
                // There will always be an empty string as the last element, it is always ignored.
                acc
            } else {
                let tmp: Vec<&str> = item.split(" '").collect();
                acc.insert(tmp[0], tmp[1]);
                acc
            }
        });

    // There should always be a G entry.
    let code = parsed_values.get("G").unwrap().to_string();
    let sectors = parsed_values.get("A").map(|s| s.to_string());

    (code, sectors)
}

fn create_journey_platform(mut values: Vec<ParsedValue>) -> Rc<JourneyPlatform> {
    let stop_id: i32 = values.remove(0).into();
    let journey_id: i32 = values.remove(0).into();
    let unknown1: String = values.remove(0).into();
    // TODO : How to name that ?
    let pindex: i32 = values.remove(0).into();
    let hour: Option<i16> = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();

    Rc::new(JourneyPlatform::new(
        journey_id,
        Platform::create_id(stop_id, pindex),
        unknown1,
        hour,
        bit_field_id,
    ))
}

fn create_platform(mut values: Vec<ParsedValue>) -> Rc<Platform> {
    let stop_id: i32 = values.remove(0).into();
    // TODO : How to name that ?
    let pindex: i32 = values.remove(0).into();
    let (code, sectors) = parse_platform_data(values.remove(0).into());

    Rc::new(Platform::new(
        Platform::create_id(stop_id, pindex),
        code,
        sectors,
    ))
}

fn set_sloid_of_platform(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    platforms_primary_index: &HashMap<i64, Rc<Platform>>,
) {
    // The SLOID is processed only when loading LV95 coordinates.
    if coordinate_type == CoordinateType::LV95 {
        let stop_id: i32 = values.remove(0).into();
        // TODO : How to name that ?
        let pindex: i32 = values.remove(0).into();
        let sloid: String = values.remove(0).into();

        platforms_primary_index
            .get(&Platform::create_id(stop_id, pindex))
            .unwrap()
            .set_sloid(sloid);
    }
}

fn set_coordinate_of_platform(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    platforms_primary_index: &HashMap<i64, Rc<Platform>>,
) {
    let stop_id: i32 = values.remove(0).into();
    // TODO : How to name that ?
    let pindex: i32 = values.remove(0).into();
    let mut xy1: f64 = values.remove(0).into();
    let mut xy2: f64 = values.remove(0).into();
    // Altitude is not provided for platforms.
    let altitude = 0;

    if coordinate_type == CoordinateType::WGS84 {
        // WGS84 coordinates are stored in reverse order for some unknown reason.
        (xy1, xy2) = (xy2, xy1);
    }

    let coordinate = Coordinate::new(coordinate_type, xy1, xy2, altitude);
    let platform = platforms_primary_index
        .get(&Platform::create_id(stop_id, pindex))
        .unwrap();

    match coordinate_type {
        CoordinateType::LV95 => platform.set_lv95_coordinate(coordinate),
        CoordinateType::WGS84 => platform.set_wgs84_coordinate(coordinate),
    }
}