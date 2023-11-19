// ECKDATEN
use std::error::Error;

use crate::parsing::{ParsedValue, RowDefinition, RowMatcher, RowParser};

use super::{ColumnDefinition, ExpectedType, FileParser};

pub fn load_timetable_key_data() -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 1;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::new(ROW_A, RowMatcher::new(2, 3, ".", true), vec![
            ColumnDefinition::new(1, 10, ExpectedType::String),
        ]),
        RowDefinition::new(ROW_B, RowMatcher::new(2, 3, ".", false), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String),
        ]),
    ]);

    let file_parser = FileParser::new("data/ECKDATEN", row_parser)?;

    let mut data: Vec<Vec<ParsedValue>> = file_parser.parse().map(|x| x.2).collect();
    let _timetable_start = String::from(data[0].remove(0));
    let _timetable_end = String::from(data[1].remove(0));
    let _metadata = String::from(data[2].remove(0));

    Ok(())
}
