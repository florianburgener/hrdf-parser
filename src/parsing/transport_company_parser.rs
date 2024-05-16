// 4 file(s).
// File(s) read by the parser:
// BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT
use std::{error::Error, rc::Rc};

use regex::Regex;

use crate::{
    models::{Language, Model, ResourceIndex, TransportCompany},
    parsing::{ColumnDefinition, ExpectedType, FastRowMatcher, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::{FileParser, ParsedValue};

pub fn parse() -> Result<SimpleResourceStorage<TransportCompany>, Box<dyn Error>> {
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

    let mut rows = Vec::new();

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_B => rows.push(create_instance(values)),
        _ => return,
    });

    let pk_index = TransportCompany::create_pk_index(&rows);

    load_designations(&pk_index, Language::German)?;
    load_designations(&pk_index, Language::English)?;
    load_designations(&pk_index, Language::French)?;
    load_designations(&pk_index, Language::Italian)?;

    Ok(SimpleResourceStorage::new(rows))
}

fn load_designations(
    pk_index: &ResourceIndex<TransportCompany>,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the short name, long name and full name in a specific language.
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
        ROW_A => set_designations(values, pk_index, language),
        _ => return,
    });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<TransportCompany> {
    let id: i32 = values.remove(0).into();
    let administrations = values.remove(0).into();

    let administrations = parse_administrations(administrations);

    Rc::new(TransportCompany::new(id, administrations))
}

fn set_designations(
    mut values: Vec<ParsedValue>,
    pk_index: &ResourceIndex<TransportCompany>,
    language: Language,
) {
    let id: i32 = values.remove(0).into();
    let designations = values.remove(0).into();

    let (short_name, long_name, full_name) = parse_designations(designations);

    let transport_company = pk_index.get(&id).unwrap();
    transport_company.set_short_name(language, &short_name);
    transport_company.set_long_name(language, &long_name);
    transport_company.set_full_name(language, &full_name);
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn parse_administrations(administrations: String) -> Vec<String> {
    administrations
        .split_whitespace()
        .map(|s| s.to_owned())
        .collect()
}

fn parse_designations(designations: String) -> (String, String, String) {
    let re = Regex::new(r" ?(K|L|V) ").unwrap();
    let designations: Vec<String> = re
        .split(&designations)
        .map(|s| s.chars().filter(|&c| c != '"').collect())
        .collect();

    let short_name = designations[0].to_owned();
    let long_name = designations[1].to_owned();
    let full_name = designations[2].to_owned();

    (short_name, long_name, full_name)
}
