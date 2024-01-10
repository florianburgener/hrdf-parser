// 1 file(s).
// File(s) read by the parser:
// FEIERTAG => Format does not match the standard.
use std::{collections::HashMap, error::Error, rc::Rc};

use chrono::NaiveDate;

use crate::{
    models::{Holiday, HolidayCollection},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
};

use super::ParsedValue;

pub fn parse() -> Result<HolidayCollection, Box<dyn Error>> {
    println!("Parsing FEIERTAG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Holiday instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 10, ExpectedType::String),
            ColumnDefinition::new(12, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/FEIERTAG", row_parser)?;

    let mut holidays = Vec::new();

    for (_, _, values) in file_parser.parse() {
        holidays.push(create_holiday(values)?);
    }

    Ok(holidays)
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_name(raw_name: String) -> HashMap<String, String> {
    raw_name
        .split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| {
            let mut parts = s.split('<');

            let v = parts.next().unwrap().to_string();
            let k = parts.next().unwrap().to_string();

            (k, v)
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(k, v);
            acc
        })
}

fn create_holiday(mut values: Vec<ParsedValue>) -> Result<Rc<Holiday>, Box<dyn Error>> {
    let raw_date: String = values.remove(0).into();
    let raw_name: String = values.remove(0).into();

    let date = NaiveDate::parse_from_str(&raw_date, "%d.%m.%Y")?;
    let name = parse_name(raw_name);

    Ok(Rc::new(Holiday::new(date, name)))
}
