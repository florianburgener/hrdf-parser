// 1 file(s).
// File(s) read by the parser:
// DURCHBI
use std::{error::Error, rc::Rc};

use crate::{
    models::{AutoIncrement, Journey, Model, ResourceIndex, ThroughService},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse(
    journeys_legacy_pk_index: &ResourceIndex<Journey, (i32, String)>,
) -> Result<SimpleResourceStorage<ThroughService>, Box<dyn Error>> {
    println!("Parsing DURCHBI...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a ThroughService instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 6, ExpectedType::Integer32),
            ColumnDefinition::new(8, 13, ExpectedType::String),
            ColumnDefinition::new(15, 21, ExpectedType::Integer32),
            ColumnDefinition::new(23, 28, ExpectedType::Integer32),
            ColumnDefinition::new(30, 35, ExpectedType::String),
            ColumnDefinition::new(37, 42, ExpectedType::Integer32), // Should be INT16 according to the standard. The standard contains an error. The correct type is INT32.
            ColumnDefinition::new(44, 50, ExpectedType::OptionInteger32),
        ]),
    ]);
    let parser = FileParser::new("data/DURCHBI", row_parser)?;

    let auto_increment = AutoIncrement::new();

    let rows = parser
        .parse()
        .filter_map(|(_, _, values)| {
            Some(create_instance(
                values,
                &auto_increment,
                journeys_legacy_pk_index,
            ))
        })
        .collect();

    Ok(SimpleResourceStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    journeys_legacy_pk_index: &ResourceIndex<Journey, (i32, String)>,
) -> Rc<ThroughService> {
    let journey_1_id: i32 = values.remove(0).into();
    let journey_1_administration: String = values.remove(0).into();
    let journey_1_stop_id: i32 = values.remove(0).into();
    let journey_2_id: i32 = values.remove(0).into();
    let journey_2_administration: String = values.remove(0).into();
    let bit_field_id: i32 = values.remove(0).into();
    let journey_2_stop_id: Option<i32> = values.remove(0).into();

    let journey_1_id = journeys_legacy_pk_index
        .get(&(journey_1_id, journey_1_administration))
        .unwrap()
        .id();

    let journey_2_id = journeys_legacy_pk_index
        .get(&(journey_2_id, journey_2_administration))
        .unwrap()
        .id();

    Rc::new(ThroughService::new(
        auto_increment.next(),
        journey_1_id,
        journey_1_stop_id,
        journey_2_id,
        journey_2_stop_id,
        bit_field_id,
    ))
}
