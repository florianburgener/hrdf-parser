// 1 file(s).
// File(s) read by the parser:
// FPLAN
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{
        AutoIncrement, Journey, JourneyMetadataEntry, JourneyMetadataType, ResourceIndex,
        TransportType,
    },
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, RowDefinition, RowParser,
    },
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse(
    transport_type_legacy_primary_index: ResourceIndex<TransportType, String>,
) -> Result<
    (
        SimpleResourceStorage<Journey>,
        ResourceIndex<Journey, (i32, String)>,
    ),
    Box<dyn Error>,
> {
    println!("Parsing FPLAN...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    // const ROW_C: i32 = 3;
    // const ROW_D: i32 = 4;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Journey instance.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(1, 2, "*Z", true)), vec![
            ColumnDefinition::new(4, 9, ExpectedType::Integer32),
            ColumnDefinition::new(11, 16, ExpectedType::String),
        ]),
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 2, "*G", true)), vec![
            ColumnDefinition::new(4, 6, ExpectedType::String),
            ColumnDefinition::new(8, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 22, ExpectedType::Integer32),
        ]),
    ]);
    let file_parser = FileParser::new("data/FPLAN", row_parser)?;

    let mut rows = Vec::new();
    let mut legacy_primary_index = HashMap::new();
    let mut current_instance = Rc::new(Journey::default());
    let auto_increment = AutoIncrement::new();

    for (id, _, values) in file_parser.parse() {
        match id {
            ROW_A => {
                if auto_increment.value() == 1 {
                    break;
                }

                let (instance, k) = create_instance(values, &auto_increment);
                legacy_primary_index.insert(k, Rc::clone(&instance));
                rows.push(Rc::clone(&instance));
                current_instance = instance;
            }
            ROW_B => {
                set_transport_type(values, &current_instance, &transport_type_legacy_primary_index);
                break;
            }
            _ => break, //unreachable!(),
        }
    }

    println!("{:?}", rows);
    Ok((SimpleResourceStorage::new(rows), legacy_primary_index))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
) -> (Rc<Journey>, (i32, String)) {
    let legacy_id: i32 = values.remove(0).into();
    let administration: String = values.remove(0).into();

    let instance = Rc::new(Journey::new(
        auto_increment.next(),
        administration.to_owned(),
    ));
    (instance, (legacy_id, administration))
}

fn set_transport_type(mut values: Vec<ParsedValue>, journey: &Rc<Journey>, transport_type_legacy_primary_index: ResourceIndex<TransportType, String>) {
    let designation: String = values.remove(0).into();
    let from_stop_id: i32 = values.remove(0).into();
    let until_stop_id: i32 = values.remove(0).into();

    journey.add_metadata_entry(
        JourneyMetadataType::TransportType,
        JourneyMetadataEntry::new(from_stop_id, until_stop_id, Some(0), None, None, None, None),
    );
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------
