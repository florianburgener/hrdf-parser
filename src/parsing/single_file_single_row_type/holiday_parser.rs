// 1 file(s).
// File(s) read by the parser:
// FEIERTAG
use std::{collections::HashMap, error::Error, str::FromStr};

use chrono::NaiveDate;

use crate::{
    models::{Holiday, Language, Model},
    parsing::{ColumnDefinition, ExpectedType, FileParser, ParsedValue, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
    utils::AutoIncrement,
};

pub fn parse() -> Result<SimpleResourceStorage<Holiday>, Box<dyn Error>> {
    println!("Parsing FEIERTAG...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Holiday instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 10, ExpectedType::String),
            ColumnDefinition::new(12, -1, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new("data/FEIERTAG", row_parser)?;

    let auto_increment = AutoIncrement::new();

    let data = parser
        .parse()
        .map(|(_, _, values)| create_instance(values, &auto_increment))
        .collect();
    let data = Holiday::vec_to_map(data);

    Ok(SimpleResourceStorage::new(data))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>, auto_increment: &AutoIncrement) -> Holiday {
    let date: String = values.remove(0).into();
    let name_translations: String = values.remove(0).into();

    let date = NaiveDate::parse_from_str(&date, "%d.%m.%Y").unwrap();
    let name = parse_name_translations(name_translations);

    Holiday::new(auto_increment.next(), date, name)
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_name_translations(name_translations: String) -> HashMap<Language, String> {
    name_translations
        .split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| {
            let mut parts = s.split('<');

            let v = parts.next().unwrap().to_string();
            let k = parts.next().unwrap().to_string();

            (k, v)
        })
        .fold(HashMap::new(), |mut acc, (k, v)| {
            acc.insert(Language::from_str(&k).unwrap(), v);
            acc
        })
}
