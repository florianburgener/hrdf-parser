use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::BitField,
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
};

use super::ParsedValue;

pub fn load_bit_fields() -> Result<(Vec<Rc<BitField>>, HashMap<i32, Rc<BitField>>), Box<dyn Error>>
{
    println!("Parsing BITFIELD...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a BitField instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 6, ExpectedType::Integer32), //
            ColumnDefinition::new(8, 103, ExpectedType::String),  //
        ]),
    ]);
    let file_parser = FileParser::new("data/BITFELD", row_parser)?;

    let bit_fields = file_parser
        .parse()
        .map(|(_, _, values)| create_bit_field(values))
        .collect();

    let bit_fields_primary_index = create_bit_fields_primary_index(&bit_fields);

    Ok((bit_fields, bit_fields_primary_index))
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_bit_fields_primary_index(bit_fields: &Vec<Rc<BitField>>) -> HashMap<i32, Rc<BitField>> {
    bit_fields.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id(), Rc::clone(item));
        acc
    })
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

fn create_bit_field(mut values: Vec<ParsedValue>) -> Rc<BitField> {
    let id: i32 = values.remove(0).into();
    let values: String = values.remove(0).into();

    let values = parse_values(values);

    Rc::new(BitField::new(id, values))
}
