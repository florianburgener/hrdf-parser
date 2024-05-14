// --$--
// 1 file(s).
// File(s) read by the parser:
// RICHTUNG
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::Direction,
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleDataStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleDataStorage<Direction>, Box<dyn Error>> {
    println!("Parsing RICHTUNG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Direction instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::String),
            ColumnDefinition::new(9, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/RICHTUNG", row_parser)?;

    let rows: Vec<Rc<Direction>>;
    let mut legacy_primary_index = HashMap::new();
    let mut next_id = 1;

    rows = file_parser
        .parse()
        .map(|(_, _, values)| {
            let instance = create_instance(values, next_id);
            legacy_primary_index.insert(instance.legacy_id().to_owned(), Rc::clone(&instance));
            next_id += 1;
            instance
        })
        .collect();

    Ok(SimpleDataStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>, id: i32) -> Rc<Direction> {
    let legacy_id: String = values.remove(0).into();
    let name: String = values.remove(0).into();

    Rc::new(Direction::new(id, legacy_id, name))
}
