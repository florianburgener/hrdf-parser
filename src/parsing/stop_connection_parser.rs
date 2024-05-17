// 0.5 file(s).
// File(s) read by the parser:
// METABHF
use std::{error::Error, rc::Rc};

use crate::{
    models::{Attribute, AutoIncrement, Model, ResourceIndex, StopConnection},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        RowDefinition, RowParser,
    },
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse(
    attribute_legacy_pk_index: &ResourceIndex<Attribute, String>,
) -> Result<SimpleResourceStorage<StopConnection>, Box<dyn Error>> {
    println!("Parsing METABHF 1/2...");
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
    let mut current_instance = Rc::new(StopConnection::default());

    let rows = parser
        .parse()
        .filter_map(|(id, _, mut values)| {
            match id {
                ROW_A => {
                    let instance = create_instance(values, &auto_increment);
                    current_instance = Rc::clone(&instance);
                    return Some(instance);
                }
                ROW_B => {
                    let attribute_designation: String = values.remove(0).into();
                    current_instance.add_attribute(
                        attribute_legacy_pk_index
                            .get(&attribute_designation)
                            .unwrap()
                            .id(),
                    );
                }
                ROW_C => (),
                _ => unreachable!(),
            };
            None
        })
        .collect();

    Ok(SimpleResourceStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
) -> Rc<StopConnection> {
    let stop_id_1: i32 = values.remove(0).into();
    let stop_id_2: i32 = values.remove(0).into();
    let duration: i16 = values.remove(0).into();

    Rc::new(StopConnection::new(
        auto_increment.next(),
        stop_id_1,
        stop_id_2,
        duration,
    ))
}
