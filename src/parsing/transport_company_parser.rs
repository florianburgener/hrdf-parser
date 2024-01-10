// File(s) read by the parser:
// BETRIEB_DE => Format matches the standard.
// BETRIEB_EN => Format matches the standard.
// BETRIEB_FR => Format matches the standard.
// BETRIEB_IT => Format matches the standard.
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{
        Language, TransportCompany, TransportCompanyCollection, TransportCompanyPrimaryIndex,
    },
    parsing::{ColumnDefinition, ExpectedType, FastRowMatcher, RowDefinition, RowParser},
};

use super::{FileParser, ParsedValue};

pub fn parse() -> Result<(TransportCompanyCollection, TransportCompanyPrimaryIndex), Box<dyn Error>> {
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

    let mut transport_companies: Vec<Rc<TransportCompany>> = Vec::new();

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_B => transport_companies.push(create_transport_company(values)),
        _ => return,
    });

    let transport_companies_primary_index =
        create_transport_companies_primary_index(&transport_companies);

    load_short_name_long_name_full_name_translations(
        &transport_companies_primary_index,
        Language::German,
    )?;
    load_short_name_long_name_full_name_translations(
        &transport_companies_primary_index,
        Language::English,
    )?;
    load_short_name_long_name_full_name_translations(
        &transport_companies_primary_index,
        Language::French,
    )?;
    load_short_name_long_name_full_name_translations(
        &transport_companies_primary_index,
        Language::Italian,
    )?;

    Ok((transport_companies, transport_companies_primary_index))
}

fn load_short_name_long_name_full_name_translations(
    transport_companies_primary_index: &TransportCompanyPrimaryIndex,
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
        ROW_A => {
            set_short_name_long_name_full_name(values, transport_companies_primary_index, language)
        }
        _ => return,
    });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_transport_companies_primary_index(
    transport_companies: &TransportCompanyCollection,
) -> TransportCompanyPrimaryIndex {
    transport_companies
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.id(), Rc::clone(item));
            acc
        })
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn create_transport_company(mut values: Vec<ParsedValue>) -> Rc<TransportCompany> {
    let id: i32 = values.remove(0).into();
    let administration: String = values.remove(0).into();

    Rc::new(TransportCompany::new(id, vec![administration]))
}

fn set_short_name_long_name_full_name(
    mut values: Vec<ParsedValue>,
    transport_companies_primary_index: &TransportCompanyPrimaryIndex,
    language: Language,
) {
    let id: i32 = values.remove(0).into();
    let raw_data: String = values.remove(0).into();
    // TODO

    let transport_company = transport_companies_primary_index.get(&id).unwrap();
    transport_company.set_short_name(language, "");
    transport_company.set_long_name(language, "");
    transport_company.set_full_name(language, "");
}
