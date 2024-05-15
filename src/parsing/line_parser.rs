// 1 file(s).
// File(s) read by the parser:
// LINIE
use std::{error::Error, rc::Rc};

use crate::{
    models::{Color, Line},
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, RowDefinition, RowParser,
    },
    storage::SimpleDataStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleDataStorage<Line>, Box<dyn Error>> {
    println!("Parsing LINIE...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Line instance.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(9, 1, "K", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(11, -1, ExpectedType::String),
        ]),
        // This row contains the short name.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(9, 3, "N T", true)), vec![
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ]),
        // This row contains the text color.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(9, 1, "F", true)), vec![
            ColumnDefinition::new(11, 13, ExpectedType::Integer16),
            ColumnDefinition::new(15, 17, ExpectedType::Integer16),
            ColumnDefinition::new(19, 21, ExpectedType::Integer16),
        ]),
        // This row contains the background color.
        RowDefinition::new(ROW_D, Box::new(FastRowMatcher::new(9, 1, "B", true)), vec![
            ColumnDefinition::new(11, 13, ExpectedType::Integer16),
            ColumnDefinition::new(15, 17, ExpectedType::Integer16),
            ColumnDefinition::new(19, 21, ExpectedType::Integer16),
        ]),
    ]);
    let file_parser = FileParser::new("data/LINIE", row_parser)?;

    let mut rows = Vec::new();
    let mut current_instance = Rc::new(Line::default());

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => {
            // Using this method, it is assumed that the following lines of types B, C and D are related to current_instance.
            current_instance = create_instance(values);
            rows.push(Rc::clone(&current_instance));
        }
        ROW_B => set_short_name(values, &current_instance),
        ROW_C => set_text_color(values, &current_instance),
        ROW_D => set_background_color(values, &current_instance),
        _ => unreachable!(),
    });

    Ok(SimpleDataStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<Line> {
    let id: i32 = values.remove(0).into();
    let name: String = values.remove(0).into();

    Rc::new(Line::new(id, name))
}

fn set_short_name(mut values: Vec<ParsedValue>, line: &Rc<Line>) {
    let short_name: String = values.remove(0).into();

    line.set_short_name(short_name);
}

fn set_text_color(mut values: Vec<ParsedValue>, line: &Rc<Line>) {
    let r: i16 = values.remove(0).into();
    let g: i16 = values.remove(0).into();
    let b: i16 = values.remove(0).into();

    line.set_text_color(Color::new(r, g, b));
}

fn set_background_color(mut values: Vec<ParsedValue>, line: &Rc<Line>) {
    let r: i16 = values.remove(0).into();
    let g: i16 = values.remove(0).into();
    let b: i16 = values.remove(0).into();

    line.set_background_color(Color::new(r, g, b));
}
