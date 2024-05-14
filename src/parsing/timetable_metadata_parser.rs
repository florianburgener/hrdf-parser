// --$--
// 1 file(s).
// File(s) read by the parser:
// ECKDATEN
use std::{error::Error, rc::Rc};

use chrono::NaiveDate;

use crate::{
    models::TimetableMetadata,
    parsing::{AdvancedRowMatcher, FastRowMatcher, ParsedValue, RowDefinition, RowParser},
    storage::SimpleDataStorage,
};

use super::{ColumnDefinition, ExpectedType, FileParser};

pub fn parse() -> Result<SimpleDataStorage<TimetableMetadata>, Box<dyn Error>> {
    println!("Parsing ECKDATEN...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the period start/end date in which timetables are effective.
        RowDefinition::new(ROW_A, Box::new(AdvancedRowMatcher::new(r"^[0-9]{2}.[0-9]{2}.[0-9]{4}$")?), vec![
            ColumnDefinition::new(1, 10, ExpectedType::String),
        ]),
        // This row contains the name, the creation date, the version and the provider of the timetable.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 0, "", true)), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String),
        ]),
    ]);

    let file_parser = FileParser::new("data/ECKDATEN", row_parser)?;

    let mut data: Vec<ParsedValue> = file_parser
        .parse()
        .map(|(_, _, mut values)| values.remove(0))
        .collect();

    let start_date: String = data.remove(0).into();
    let end_date: String = data.remove(0).into();
    let metadata: String = data.remove(0).into();

    let start_date = NaiveDate::parse_from_str(&start_date, "%d.%m.%Y")?;
    let end_date = NaiveDate::parse_from_str(&end_date, "%d.%m.%Y")?;
    let metadata: Vec<String> = metadata.split('$').map(String::from).collect();

    let rows: Vec<Rc<TimetableMetadata>>;
    let mut next_id = 1;

    rows = vec![
        ("start_date", start_date.to_string()),
        ("end_date", end_date.to_string()),
        ("name", metadata[0].to_owned()),
        ("created_at", metadata[1].to_owned()),
        ("version", metadata[2].to_owned()),
        ("provider", metadata[3].to_owned()),
    ]
    .iter()
    .map(|(key, value)| {
        let instance = Rc::new(TimetableMetadata::new(
            next_id,
            key.to_string(),
            value.to_owned(),
        ));
        next_id += 1;
        instance
    })
    .collect();

    Ok(SimpleDataStorage::new(rows))
}
