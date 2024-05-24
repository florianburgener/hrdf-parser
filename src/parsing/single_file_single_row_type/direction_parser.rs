// 1 file(s).
// File(s) read by the parser:
// RICHTUNG
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Direction, ResourceIndex},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

pub fn parse() -> Result<
    (
        SimpleResourceStorage<Direction>,
        ResourceIndex<String, Direction>,
    ),
    Box<dyn Error>,
> {
    println!("Parsing RICHTUNG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Direction instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::String),
            ColumnDefinition::new(9, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new("data/RICHTUNG", row_parser)?;

    let mut original_primary_index = HashMap::new();

    let rows = parser
        .parse()
        .map(|(_, _, values)| {
            let (instance, k) = create_instance(values);
            original_primary_index.insert(k, Rc::clone(&instance));
            instance
        })
        .collect();

    Ok((SimpleResourceStorage::new(rows), original_primary_index))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> (Rc<Direction>, String) {
    let id_str: String = values.remove(0).into();
    let name: String = values.remove(0).into();

    let id = remove_first_char(&id_str).parse::<i32>().unwrap();

    (Rc::new(Direction::new(id, name)), id_str)
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn remove_first_char(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
