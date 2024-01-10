// 1 file(s).
// File(s) read by the parser:
// RICHTUNG => it's unclear whether the format matches the standard or not.
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Direction, DirectionCollection, DirectionPrimaryIndex},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::DirectionData,
};

use super::ParsedValue;

pub fn parse() -> Result<DirectionData, Box<dyn Error>> {
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

    let rows: Vec<Rc<Direction>> = file_parser
        .parse()
        .map(|(_, _, values)| create_instance(values))
        .collect();

    let primary_index = create_instances_primary_index(&rows);

    Ok(DirectionData::new(rows, primary_index))
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_instances_primary_index(rows: &DirectionCollection) -> DirectionPrimaryIndex {
    rows.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id().to_owned(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<Direction> {
    let id: String = values.remove(0).into();
    let name: String = values.remove(0).into();

    Rc::new(Direction::new(id, name))
}
