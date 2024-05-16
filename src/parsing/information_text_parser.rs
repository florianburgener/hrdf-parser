// 4 file(s).
// File(s) read by the parser:
// INFOTEXT_DE, INFOTEXT_EN, INFOTEXT_FR, INFOTEXT_IT
use std::{error::Error, rc::Rc};

use crate::{
    models::{InformationText, Language, Model, ResourceIndex},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse() -> Result<SimpleResourceStorage<InformationText>, Box<dyn Error>> {
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

    let pk_index = InformationText::create_pk_index(&rows);

    load_content(&pk_index, Language::German)?;
    load_content(&pk_index, Language::English)?;
    load_content(&pk_index, Language::French)?;
    load_content(&pk_index, Language::Italian)?;

    Ok(SimpleResourceStorage::new(rows))
}

fn load_content(
    pk_index: &ResourceIndex<InformationText>,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row contains the content in a specific language.
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
        .for_each(|(_, _, values)| set_content(values, pk_index, language));

    Ok(())
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
    pk_index: &ResourceIndex<InformationText>,
    language: Language,
) {
    let id: i32 = values.remove(0).into();
    let description: String = values.remove(0).into();

    pk_index
        .get(&id)
        .unwrap()
        .set_content(language, &description);
}
