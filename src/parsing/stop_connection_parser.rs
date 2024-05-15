// 0.5 file(s).
// File(s) read by the parser:
// METABHF
use std::{error::Error, rc::Rc};

use crate::{
    models::StopConnection,
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        RowDefinition, RowParser,
    },
    storage::SimpleDataStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleDataStorage<StopConnection>, Box<dyn Error>> {
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
    let file_parser = FileParser::new("data/METABHF", row_parser)?;

    let mut rows: Vec<Rc<StopConnection>> = Vec::new();
    let mut next_id = 1;
    let mut current_instance = Rc::new(StopConnection::default());

    file_parser
        .parse()
        .for_each(|(id, _, mut values)| match id {
            ROW_A => {
                current_instance = create_instance(values, next_id);
                rows.push(Rc::clone(&current_instance));
                next_id += 1;
            }
            ROW_B => {
                let attribute: String = values.remove(0).into();
                current_instance.add_attribute(attribute);
            }
            ROW_C => return,
            _ => unreachable!(),
        });

    Ok(SimpleDataStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>, id: i32) -> Rc<StopConnection> {
    let stop_id_1: i32 = values.remove(0).into();
    let stop_id_2: i32 = values.remove(0).into();
    let duration: i16 = values.remove(0).into();

    Rc::new(StopConnection::new(id, stop_id_1, stop_id_2, duration))
}
