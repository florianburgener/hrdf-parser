// 3 file(s).
// File(s) read by the parser:
// GLEIS, GLEIS_LV95, GLEIS_WGS
// ---
// Note: this parser collects both the Platform and JourneyPlatform resources.
use std::error::Error;

use rustc_hash::FxHashMap;

use crate::{
    models::{Coordinate, CoordinateType, JourneyPlatform, Model, Platform},
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, ParsedValue, RowDefinition,
        RowParser,
    },
    storage::SimpleResourceStorage,
    utils::{create_time_from_value, AutoIncrement},
};

pub fn parse(
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
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
    let mut platforms = Vec::new();
    let mut platforms_pk_type_converter = FxHashMap::default();

    let mut bytes_offset = 0;
    let mut journey_platform = Vec::new();

    for (id, bytes_read, values) in parser.parse() {
        match id {
            ROW_A => {
                bytes_offset += bytes_read;
                journey_platform.push(values);
            }
            ROW_B => {
                platforms.push(create_platform(
                    values,
                    &auto_increment,
                    &mut platforms_pk_type_converter,
                ));
            }
            _ => unreachable!(),
        }
    }

    let mut platforms = Platform::vec_to_map(platforms);

    let journey_platform = journey_platform
        .into_iter()
        .map(|values| {
            create_journey_platform(
                values,
                journeys_pk_type_converter,
                &platforms_pk_type_converter,
            )
        })
        .collect();
    let journey_platform = JourneyPlatform::vec_to_map(journey_platform);

    println!("Parsing GLEIS_LV95...");
    #[rustfmt::skip]
    load_coordinates_for_platforms(CoordinateType::LV95, bytes_offset, &platforms_pk_type_converter, &mut platforms)?;
    println!("Parsing GLEIS_WGS84...");
    #[rustfmt::skip]
    load_coordinates_for_platforms(CoordinateType::WGS84, bytes_offset, &platforms_pk_type_converter, &mut platforms)?;

    Ok((
        SimpleResourceStorage::new(journey_platform),
        SimpleResourceStorage::new(platforms),
    ))
}

fn load_coordinates_for_platforms(
    coordinate_type: CoordinateType,
    bytes_offset: u64,
    pk_type_converter: &FxHashMap<(i32, i32), i32>,
    data: &mut FxHashMap<i32, Platform>,
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
        ROW_B => platform_set_sloid(values, coordinate_type, pk_type_converter, data),
        ROW_C => platform_set_coordinate(values, coordinate_type, pk_type_converter, data),
        _ => unreachable!(),
    });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn create_journey_platform(
    mut values: Vec<ParsedValue>,
    journeys_pk_type_converter: &FxHashMap<(i32, String), i32>,
    platforms_pk_type_converter: &FxHashMap<(i32, i32), i32>,
) -> JourneyPlatform {
    let stop_id: i32 = values.remove(0).into();
    let journey_id: i32 = values.remove(0).into();
    let administration: String = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let time: Option<i32> = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();

    let journey_id = *journeys_pk_type_converter
        .get(&(journey_id, administration))
        .unwrap();

    let platform_id = *platforms_pk_type_converter.get(&(stop_id, index)).unwrap();

    let time = time.map(|x| create_time_from_value(x as u32));

    JourneyPlatform::new(journey_id, platform_id, time, bit_field_id)
}

fn create_platform(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    platforms_pk_type_converter: &mut FxHashMap<(i32, i32), i32>,
) -> Platform {
    let stop_id: i32 = values.remove(0).into();
    let index: i32 = values.remove(0).into();
    let platform_data: String = values.remove(0).into();

    let id = auto_increment.next();
    let (code, sectors) = parse_platform_data(platform_data);

    platforms_pk_type_converter.insert((stop_id, index), id);
    Platform::new(id, code, sectors, stop_id)
}

fn platform_set_sloid(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    pk_type_converter: &FxHashMap<(i32, i32), i32>,
    data: &mut FxHashMap<i32, Platform>,
) {
    // The SLOID is processed only when loading LV95 coordinates.
    if coordinate_type == CoordinateType::LV95 {
        let stop_id: i32 = values.remove(0).into();
        let index: i32 = values.remove(0).into();
        let sloid: String = values.remove(0).into();

        data.get_mut(&pk_type_converter.get(&(stop_id, index)).unwrap())
            .unwrap()
            .set_sloid(sloid);
    }
}

fn platform_set_coordinate(
    mut values: Vec<ParsedValue>,
    coordinate_type: CoordinateType,
    pk_type_converter: &FxHashMap<(i32, i32), i32>,
    data: &mut FxHashMap<i32, Platform>,
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
    let platform = data
        .get_mut(&pk_type_converter.get(&(stop_id, index)).unwrap())
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
        FxHashMap::default(),
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
