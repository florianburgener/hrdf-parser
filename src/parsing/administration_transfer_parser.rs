// 1 file(s).
// File(s) read by the parser:
// UMSTEIGV
use std::{error::Error, rc::Rc};

use crate::{
    models::{AdministrationTransferTime, AutoIncrement},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleResourceStorage<AdministrationTransferTime>, Box<dyn Error>> {
    println!("Parsing UMSTEIGV...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a AdministrationTransferTime instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::OptionInteger32),
            ColumnDefinition::new(9, 14, ExpectedType::String),
            ColumnDefinition::new(16, 21, ExpectedType::String),
            ColumnDefinition::new(23, 24, ExpectedType::Integer16),
        ]),
    ]);
    let parser = FileParser::new("data/UMSTEIGV", row_parser)?;

    let auto_increment = AutoIncrement::new();

    let rows = parser
        .parse()
        .filter_map(|(_, _, values)| Some(create_instance(values, &auto_increment)))
        .collect();

    Ok(SimpleResourceStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
) -> Rc<AdministrationTransferTime> {
    let stop_id: Option<i32> = values.remove(0).into();
    let administration_1: String = values.remove(0).into();
    let administration_2: String = values.remove(0).into();
    let duration: i16 = values.remove(0).into();

    Rc::new(AdministrationTransferTime::new(
        auto_increment.next(),
        stop_id,
        administration_1,
        administration_2,
        duration,
    ))
}
