// 1 file(s).
// File(s) read by the parser:
// BITFELD => Format does not match the standard (this is not explicitly stated in the SBB document).
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{BitField, BitFieldCollection, BitFieldPrimaryIndex},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::BitFieldData,
};

use super::ParsedValue;

pub fn parse() -> Result<BitFieldData, Box<dyn Error>> {
    println!("Parsing BITFELD...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a BitField instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 6, ExpectedType::Integer32),
            ColumnDefinition::new(8, 103, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/BITFELD", row_parser)?;

    let rows = file_parser
        .parse()
        .map(|(_, _, values)| create_instance(values))
        .collect();

    let primary_index = create_instances_primary_index(&rows);

    Ok(BitFieldData::new(rows, primary_index))
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_instances_primary_index(rows: &BitFieldCollection) -> BitFieldPrimaryIndex {
    rows.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<BitField> {
    let id: i32 = values.remove(0).into();
    let raw_values: String = values.remove(0).into();

    let values = parse_values(raw_values);

    Rc::new(BitField::new(id, values))
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_values(raw_values: String) -> Vec<u8> {
    raw_values
        .chars()
        .flat_map(|hex_char| {
            (0..4)
                .rev()
                .map(move |i| ((hex_char.to_digit(16).unwrap() >> i) & 0x1) as u8)
        })
        .collect()
}
