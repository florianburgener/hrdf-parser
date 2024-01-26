// 4 file(s).
// File(s) read by the parser:
// BETRIEB_DE, BETRIEB_EN, BETRIEB_FR, BETRIEB_IT
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{
        Language, TransportCompany, TransportCompanyCollection, TransportCompanyPrimaryIndex,
    },
    parsing::{ColumnDefinition, ExpectedType, FastRowMatcher, RowDefinition, RowParser},
    storage::TransportCompanyData,
};

use super::{FileParser, ParsedValue};

pub fn parse() -> Result<TransportCompanyData, Box<dyn Error>> {
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

    let primary_index = create_primary_index(&rows);

    load_short_name_long_name_full_name_translations(&primary_index, Language::German)?;
    load_short_name_long_name_full_name_translations(&primary_index, Language::English)?;
    load_short_name_long_name_full_name_translations(&primary_index, Language::French)?;
    load_short_name_long_name_full_name_translations(&primary_index, Language::Italian)?;

    Ok(TransportCompanyData::new(rows, primary_index))
}

fn load_short_name_long_name_full_name_translations(
    primary_index: &TransportCompanyPrimaryIndex,
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
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_primary_index(rows: &TransportCompanyCollection) -> TransportCompanyPrimaryIndex {
    rows.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<TransportCompany> {
    let id: i32 = values.remove(0).into();
    let administration: String = values.remove(0).into();

    Rc::new(TransportCompany::new(id, vec![administration]))
}

fn set_short_name_long_name_full_name(
    mut values: Vec<ParsedValue>,
    primary_index: &TransportCompanyPrimaryIndex,
    language: Language,
) {
    let id: i32 = values.remove(0).into();
    // // TODO : parse that.
    // let raw_data: String = values.remove(0).into();

    let transport_company = primary_index.get(&id).unwrap();
    transport_company.set_short_name(language, "");
    transport_company.set_long_name(language, "");
    transport_company.set_full_name(language, "");
}
