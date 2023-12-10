// ECKDATEN
use std::error::Error;

use chrono::NaiveDate;

use crate::{
    models::TimetableKeyData,
    parsing::{ParsedValue, RowDefinition, FastRowMatcher, RowParser},
};

use super::{ColumnDefinition, ExpectedType, FileParser};

pub fn load_timetable_key_data() -> Result<TimetableKeyData, Box<dyn Error>> {
    println!("Parsing ECKDATEN...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 1;

    // TODO : If there is a "." in column 3 for ROW_B, this code will not work.
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // Period (start/end data) in which timetables are effective.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(3, 1, ".", true)), vec![
            ColumnDefinition::new(1, 10, ExpectedType::String), // Complies with the standard.
        ]),
        // Metadata (name, version, etc.).
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(3, 1, ".", false)), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String), // Complies with the standard.
        ]),
    ]);

    let file_parser = FileParser::new("data/ECKDATEN", row_parser)?;

    let mut data: Vec<ParsedValue> = file_parser.parse().map(|(_, _, mut values)| values.remove(0)).collect();
    let raw_start_date: String = data.remove(0).into();
    let raw_end_date: String = data.remove(0).into();
    let raw_metadata: String = data.remove(0).into();

    let start_date = NaiveDate::parse_from_str(&raw_start_date, "%d.%m.%Y")?;
    let end_date = NaiveDate::parse_from_str(&raw_end_date, "%d.%m.%Y")?;
    let metada = raw_metadata.split('$').map(String::from).collect();

    let timetable_key_data = TimetableKeyData::new(start_date, end_date, metada);

    Ok(timetable_key_data)
}
