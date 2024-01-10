// 1 file(s).
// File(s) read by the parser:
// DURCHBI => Format matches the standard.
use std::{error::Error, rc::Rc};

use crate::{
    models::ThroughService,
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::ThroughServiceData,
};

use super::ParsedValue;

pub fn parse() -> Result<ThroughServiceData, Box<dyn Error>> {
    println!("Parsing DURCHBI...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a BitField instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 6, ExpectedType::Integer32),
            ColumnDefinition::new(8, 13, ExpectedType::String),
            ColumnDefinition::new(15, 21, ExpectedType::Integer32),
            ColumnDefinition::new(23, 28, ExpectedType::Integer32),
            ColumnDefinition::new(30, 35, ExpectedType::String),
            ColumnDefinition::new(37, 42, ExpectedType::Integer32), // Should be INT16 according to the standard. The standard contains an error. The correct type is INT32.
            ColumnDefinition::new(44, 50, ExpectedType::OptionInteger32),
            // TODO : "Attribut zur Markierung der Durchbindung (optional)"
        ]),
    ]);
    let file_parser = FileParser::new("data/DURCHBI", row_parser)?;

    let rows = file_parser
        .parse()
        .map(|(_, _, values)| create_instance(values))
        .collect();

    Ok(ThroughServiceData::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<ThroughService> {
    let journey_1_id: i32 = values.remove(0).into();
    let journey_1_unknown: String = values.remove(0).into();
    let journey_1_stop_id: i32 = values.remove(0).into();
    let journey_2_id: i32 = values.remove(0).into();
    let journey_2_unknown: String = values.remove(0).into();
    let bit_field_id: i32 = values.remove(0).into();
    let journey_2_stop_id: Option<i32> = values.remove(0).into();

    Rc::new(ThroughService::new(
        journey_1_id,
        journey_1_unknown,
        journey_1_stop_id,
        journey_2_id,
        journey_2_unknown,
        journey_2_stop_id,
        bit_field_id,
    ))
}
