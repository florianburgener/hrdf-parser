// 4 file(s).
// File(s) read by the parser:
// INFOTEXT_DE => Format matches the standard.
// INFOTEXT_EN => Format matches the standard.
// INFOTEXT_FR => Format matches the standard.
// INFOTEXT_IT => Format matches the standard.
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{InformationText, InformationTextCollection, InformationTextPrimaryIndex, Language},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::InformationTextData,
};

use super::ParsedValue;

pub fn parse() -> Result<InformationTextData, Box<dyn Error>> {
    println!("Parsing INFOTEXT_DE...");
    println!("Parsing INFOTEXT_EN...");
    println!("Parsing INFOTEXT_FR...");
    println!("Parsing INFOTEXT_IT...");

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a InformationText instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 9, ExpectedType::Integer32),
        ]),
    ]);
    let file_parser = FileParser::new("data/INFOTEXT_DE", row_parser)?;

    let rows = file_parser
        .parse()
        .map(|(_, _, values)| create_instance(values))
        .collect();

    let primary_index = create_primary_index(&rows);

    load_content_translation(&primary_index, Language::German)?;
    load_content_translation(&primary_index, Language::English)?;
    load_content_translation(&primary_index, Language::French)?;
    load_content_translation(&primary_index, Language::Italian)?;

    Ok(InformationTextData::new(rows, primary_index))
}

fn load_content_translation(
    primary_index: &InformationTextPrimaryIndex,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a InformationText instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 9, ExpectedType::Integer32),
            ColumnDefinition::new(11, -1, ExpectedType::String),
        ]),
    ]);
    let filename = match language {
        Language::German => "INFOTEXT_DE",
        Language::English => "INFOTEXT_EN",
        Language::French => "INFOTEXT_FR",
        Language::Italian => "INFOTEXT_IT",
    };
    let file_path = format!("data/{}", filename);
    let file_parser = FileParser::new(&file_path, row_parser)?;

    file_parser
        .parse()
        .for_each(|(_, _, values)| set_content(values, primary_index, language));

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_primary_index(rows: &InformationTextCollection) -> InformationTextPrimaryIndex {
    rows.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<InformationText> {
    let id: i32 = values.remove(0).into();

    Rc::new(InformationText::new(id))
}

fn set_content(
    mut values: Vec<ParsedValue>,
    primary_index: &InformationTextPrimaryIndex,
    language: Language,
) {
    let id: i32 = values.remove(0).into();
    let description: String = values.remove(0).into();

    primary_index
        .get(&id)
        .unwrap()
        .set_content(language, &description);
}
