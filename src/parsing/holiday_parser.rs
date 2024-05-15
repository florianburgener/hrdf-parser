// 1 file(s).
// File(s) read by the parser:
// FEIERTAG
use std::{collections::HashMap, error::Error, rc::Rc};

use chrono::NaiveDate;

use crate::{
    models::Holiday,
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleDataStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleDataStorage<Holiday>, Box<dyn Error>> {
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

    let mut rows = Vec::new();
    let mut next_id = 1;

    // A for loop is used here, as create_instance must be able to return an error.
    for (_, _, values) in file_parser.parse() {
        rows.push(create_instance(values, next_id)?);
        next_id += 1;
    }

    Ok(SimpleDataStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>, id: i32) -> Result<Rc<Holiday>, Box<dyn Error>> {
    let date: String = values.remove(0).into();
    let name_translations: String = values.remove(0).into();

    let date = NaiveDate::parse_from_str(&date, "%d.%m.%Y")?;
    let name = parse_name_translations(name_translations);

    Ok(Rc::new(Holiday::new(id, date, name)))
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_name_translations(name_translations: String) -> HashMap<String, String> {
    name_translations.split('>')
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
