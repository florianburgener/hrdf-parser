// ATTRIBUT
// Unused files: ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT
use std::error::Error;

use regex::Regex;

use crate::parsing::{
    ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowMatcher, RowParser,
};

pub fn load_attributes() -> Result<(), Box<dyn Error>> {
    println!("Parsing ATTRIBUT...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    // TODO : "Complies with the standard."
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::new(ROW_A, RowMatcher::new_with_re_only(
            Regex::new("^.{2} [0-9] [0-9 ]{3} [0-9 ]{2}$").unwrap()
        ), vec![
            // ColumnDefinition::new(1, 2, ExpectedType::String),      // Complies with the standard.
            // ColumnDefinition::new(4, 4, ExpectedType::Integer16),   // Complies with the standard.
            // ColumnDefinition::new(6, 8, ExpectedType::Integer16),   // Complies with the standard.
            // ColumnDefinition::new(10, 11, ExpectedType::Integer16), // Complies with the standard.
        ]),
        RowDefinition::new(ROW_B, RowMatcher::new(1, 1, "#", true), vec![]),
        RowDefinition::new(ROW_C, RowMatcher::new(1, 1, "<", true), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String), // Complies with the standard.
        ]),
        RowDefinition::new(ROW_D, RowMatcher::new_with_re_only(
            Regex::new("^.{2} .+$").unwrap()
        ), vec![
            ColumnDefinition::new(1, 2, ExpectedType::String),  // Complies with the standard.
            ColumnDefinition::new(4, -1, ExpectedType::String), // Complies with the standard.
        ]),
    ]);
    let file_parser = FileParser::new("data/ATTRIBUT", row_parser)?;

    for (id, _, mut values) in file_parser.parse() {
        println!("{} --- {:?}", id, values);
    }

    Ok(())
}
