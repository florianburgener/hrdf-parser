// 0.5 file(s).
// File(s) read by the parser:
// METABHF
use std::{collections::HashMap, error::Error};

use crate::{
    models::{Model, StopConnection},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        ParsedValue, RowDefinition, RowParser,
    },
    storage::SimpleResourceStorage,
    utils::AutoIncrement,
};

pub fn parse(
    attributes_pk_type_converter: &HashMap<String, i32>,
) -> Result<SimpleResourceStorage<StopConnection>, Box<dyn Error>> {
    println!("Parsing METABHF 2/2...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a StopConnection instance.
        RowDefinition::new(ROW_A, Box::new(AdvancedRowMatcher::new(r"[0-9]{7} [0-9]{7} [0-9]{3}")?), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 15, ExpectedType::Integer32),
            ColumnDefinition::new(17, 19, ExpectedType::Integer16),
        ]),
        // This row contains the attributes.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 2, "*A", true)), vec![
            ColumnDefinition::new(4, 5, ExpectedType::String),
        ]),
        // This row is ignored.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(8, 1, ":", true)), Vec::new()),
    ]);
    let parser = FileParser::new("data/METABHF", row_parser)?;

    let auto_increment = AutoIncrement::new();
    let mut data = Vec::new();

    for (id, _, values) in parser.parse() {
        match id {
            ROW_A => data.push(create_instance(values, &auto_increment)),
            _ => {
                let stop_connection = data.last_mut().unwrap();

                match id {
                    ROW_B => add_attribute(values, &stop_connection, attributes_pk_type_converter),
                    ROW_C => {}
                    _ => unreachable!(),
                }
            }
        }
    }

    let data = StopConnection::vec_to_map(data);

    Ok(SimpleResourceStorage::new(data))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>, auto_increment: &AutoIncrement) -> StopConnection {
    let stop_id_1: i32 = values.remove(0).into();
    let stop_id_2: i32 = values.remove(0).into();
    let duration: i16 = values.remove(0).into();

    StopConnection::new(auto_increment.next(), stop_id_1, stop_id_2, duration)
}

fn add_attribute(
    mut values: Vec<ParsedValue>,
    current_instance: &StopConnection,
    attributes_pk_type_converter: &HashMap<String, i32>,
) {
    let attribute_designation: String = values.remove(0).into();
    let attribute_id = *attributes_pk_type_converter
        .get(&attribute_designation)
        .unwrap();
    current_instance.add_attribute(attribute_id);
}
