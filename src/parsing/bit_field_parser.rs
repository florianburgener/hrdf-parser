// 1 file(s).
// File(s) read by the parser:
// BITFELD
use std::error::Error;

use crate::{
    models::{BitField, Model},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::ResourceStorage,
};

pub fn parse(path: &str) -> Result<ResourceStorage<BitField>, Box<dyn Error>> {
    log::info!("Parsing BITFELD...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a BitField instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 6, ExpectedType::Integer32),
            ColumnDefinition::new(8, 103, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new(&format!("{path}/BITFELD"), row_parser)?;

    let data = parser
        .parse()
        .map(|x| x.and_then(|(_, _, values)| create_instance(values)))
        .collect::<Result<Vec<_>, _>>()?;
    let data = BitField::vec_to_map(data);

    Ok(ResourceStorage::new(data))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Result<BitField, Box<dyn Error>> {
    let id: i32 = values.remove(0).into();
    let hex_number: String = values.remove(0).into();

    let bits = convert_hex_number_to_bits(hex_number)?;

    Ok(BitField::new(id, bits))
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

/// Converts a hexadecimal number into a list of where each item represents a bit.
fn convert_hex_number_to_bits(hex_number: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let result = hex_number
        .chars()
        .map(|hex_digit| {
            hex_digit
                .to_digit(16)
                .ok_or("Invalid hexadecimal digit")
                .map(|val| (0..4).rev().map(move |i| ((val >> i) & 1) as u8))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    Ok(result)
}
