// 4 file(s).
// File(s) read by the parser:
// BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT
use std::{error::Error, rc::Rc};

use regex::Regex;

use crate::{
    models::{Language, Model, PrimaryIndex, TransportCompany},
    parsing::{ColumnDefinition, ExpectedType, FastRowMatcher, RowDefinition, RowParser},
    storage::SimpleDataStorage,
};

use super::{FileParser, ParsedValue};

pub fn parse() -> Result<SimpleDataStorage<TransportCompany>, Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(7, 1, "K", true)), Vec::new()),
        // This row is used to create a TransportCompany instance.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(7, 1, ":", true)), vec![
            ColumnDefinition::new(1, 5, ExpectedType::Integer32),
            ColumnDefinition::new(9, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/BETRIEB_DE", row_parser)?;

    let mut rows: Vec<Rc<TransportCompany>> = Vec::new();

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_B => rows.push(create_instance(values)),
        _ => return,
    });

    let primary_index = TransportCompany::create_primary_index(&rows);

    load_short_name_long_name_full_name_translations(&primary_index, Language::German)?;
    load_short_name_long_name_full_name_translations(&primary_index, Language::English)?;
    load_short_name_long_name_full_name_translations(&primary_index, Language::French)?;
    load_short_name_long_name_full_name_translations(&primary_index, Language::Italian)?;

    Ok(SimpleDataStorage::new(rows))
}

fn load_short_name_long_name_full_name_translations(
    primary_index: &PrimaryIndex<TransportCompany>,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a TransportCompany instance.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(7, 1, "K", true)), vec![
            ColumnDefinition::new(1, 5, ExpectedType::Integer32),
            ColumnDefinition::new(9, -1, ExpectedType::String),
        ]),
        // This row is ignored.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(7, 1, ":", true)), Vec::new()),
    ]);
    let filename = match language {
        Language::German => "BETRIEB_DE",
        Language::English => "BETRIEB_EN",
        Language::French => "BETRIEB_FR",
        Language::Italian => "BETRIEB_IT",
    };
    let file_path = format!("data/{}", filename);
    let file_parser = FileParser::new(&file_path, row_parser)?;

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => set_short_name_long_name_full_name(values, primary_index, language),
        _ => return,
    });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<TransportCompany> {
    let id: i32 = values.remove(0).into();
    let administrations = parse_administrations(values.remove(0).into());

    Rc::new(TransportCompany::new(id, administrations))
}

fn set_short_name_long_name_full_name(
    mut values: Vec<ParsedValue>,
    primary_index: &PrimaryIndex<TransportCompany>,
    language: Language,
) {
    let id: i32 = values.remove(0).into();
    let (short_name, long_name, full_name) =
        parse_short_name_long_name_full_name(values.remove(0).into());

    let transport_company = primary_index.get(&id).unwrap();
    transport_company.set_short_name(language, &short_name);
    transport_company.set_long_name(language, &long_name);
    transport_company.set_full_name(language, &full_name);
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_administrations(raw_administrations: String) -> Vec<String> {
    raw_administrations
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}

fn parse_short_name_long_name_full_name(raw_data: String) -> (String, String, String) {
    let re = Regex::new(r"( )?(K|L|V) ").unwrap();
    let data: Vec<String> = re
        .split(&raw_data)
        .map(|s| s.chars().filter(|&c| c != '"').collect())
        .collect();

    let short_name = data[0].to_owned();
    let long_name = data[1].to_owned();
    let full_name = data[2].to_owned();

    (short_name, long_name, full_name)
}
