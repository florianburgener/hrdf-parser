// ECKDATEN
use std::error::Error;

use crate::{
    models::TimetableKeyData,
    parsing::{ParsedValue, RowDefinition, RowMatcher, RowParser},
};

use super::{ColumnDefinition, ExpectedType, FileParser};

pub fn load_timetable_key_data() -> Result<TimetableKeyData, Box<dyn Error>> {
    println!("Parsing ECKDATEN...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 1;

    // TODO : If there is a "." in column 3 for ROW_B, this code will not work.
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::new(ROW_A, RowMatcher::new(3, 1, ".", true), vec![
            ColumnDefinition::new(1, 10, ExpectedType::String), // Complies with the standard.
        ]),
        RowDefinition::new(ROW_B, RowMatcher::new(3, 1, ".", false), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String), // Complies with the standard.
        ]),
    ]);

    let file_parser = FileParser::new("data/ECKDATEN", row_parser)?;

    let mut data: Vec<ParsedValue> = file_parser.parse().map(|mut x| x.2.remove(0)).collect();
    let timetable_start: String = data.remove(0).into();
    let timetable_end: String = data.remove(0).into();
    let metadata: String = data.remove(0).into();

    let metada = metadata.split('$').map(String::from).collect();

    let timetable_key_data = TimetableKeyData::new(timetable_start, timetable_end, metada);

    Ok(timetable_key_data)
}
