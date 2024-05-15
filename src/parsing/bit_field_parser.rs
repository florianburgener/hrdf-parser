// 1 file(s).
// File(s) read by the parser:
// BITFELD
use std::{error::Error, rc::Rc};

use crate::{
    models::BitField,
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleResourceStorage<BitField>, Box<dyn Error>> {
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

    Ok(SimpleResourceStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<BitField> {
    let id: i32 = values.remove(0).into();
    let hex_number: String = values.remove(0).into();

    let bits = convert_hex_number_to_bits(hex_number);

    Rc::new(BitField::new(id, bits))
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

/// Converts a hexadecimal number into a list of where each item represents a bit.
fn convert_hex_number_to_bits(hex_number: String) -> Vec<u8> {
    hex_number.chars()
        .flat_map(|hex_digit| {
            (0..4)
                .rev()
                .map(move |i| ((hex_digit.to_digit(16).unwrap() >> i) & 0x1) as u8)
        })
        .collect()
}
