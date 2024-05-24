// 1 file(s).
// File(s) read by the parser:
// UMSTEIGZ
use std::{error::Error, rc::Rc};

use crate::{
    models::{Journey, Model, ResourceIndex, TransferTimeJourney},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
    utils::AutoIncrement,
};

pub fn parse(
    journeys_original_primary_index: &ResourceIndex<(i32, String), Journey>,
) -> Result<SimpleResourceStorage<TransferTimeJourney>, Box<dyn Error>> {
    println!("Parsing UMSTEIGZ...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a JourneyTransferTime instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::Integer32),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            ColumnDefinition::new(23, 28, ExpectedType::Integer32),
            ColumnDefinition::new(30, 35, ExpectedType::String),
            ColumnDefinition::new(37, 39, ExpectedType::Integer16),
            ColumnDefinition::new(40, 40, ExpectedType::String),
            ColumnDefinition::new(42, 47, ExpectedType::OptionInteger32),
        ]),
    ]);
    let parser = FileParser::new("data/UMSTEIGZ", row_parser)?;

    let auto_increment = AutoIncrement::new();

    let rows = parser
        .parse()
        .map(|(_, _, values)| {
            create_instance(values, &auto_increment, journeys_original_primary_index)
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
    journeys_original_primary_index: &ResourceIndex<(i32, String), Journey>,
) -> Rc<TransferTimeJourney> {
    let stop_id: i32 = values.remove(0).into();
    let journey_id_1: i32 = values.remove(0).into();
    let administration_1: String = values.remove(0).into();
    let journey_id_2: i32 = values.remove(0).into();
    let administration_2: String = values.remove(0).into();
    let duration: i16 = values.remove(0).into();
    let is_guaranteed: String = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();

    let journey_id_1 = journeys_original_primary_index
        .get(&(journey_id_1, administration_1))
        .unwrap()
        .id();

    let journey_id_2 = journeys_original_primary_index
        .get(&(journey_id_2, administration_2))
        .unwrap()
        .id();

    let is_guaranteed = is_guaranteed == "!";

    Rc::new(TransferTimeJourney::new(
        auto_increment.next(),
        stop_id,
        journey_id_1,
        journey_id_2,
        duration,
        is_guaranteed,
        bit_field_id,
    ))
}
