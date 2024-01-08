// File(s) read by the parser:
// INFOTEXT_DE => Format matches the standard.
// INFOTEXT_EN => Format matches the standard.
// INFOTEXT_FR => Format matches the standard.
// INFOTEXT_IT => Format matches the standard.
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{InformationText, InformationTextCollection, InformationTextPrimaryIndex, Language},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
};

use super::ParsedValue;

pub fn load_information_texts(
) -> Result<(InformationTextCollection, InformationTextPrimaryIndex), Box<dyn Error>> {
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a InformationText instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 9, ExpectedType::Integer32),
        ]),
    ]);
    let file_parser = FileParser::new("data/INFOTEXT_DE", row_parser)?;

    let information_texts = file_parser
        .parse()
        .map(|(_, _, values)| create_information_text(values))
        .collect();

    let information_texts_primary_index =
        create_information_texts_primary_index(&information_texts);

    load_content_translation(&information_texts_primary_index, Language::German)?;
    load_content_translation(&information_texts_primary_index, Language::English)?;
    load_content_translation(&information_texts_primary_index, Language::French)?;
    load_content_translation(&information_texts_primary_index, Language::Italian)?;

    Ok((information_texts, information_texts_primary_index))
}

fn load_content_translation(
    information_texts_primary_index: &InformationTextPrimaryIndex,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    let filename = match language {
        Language::German => "INFOTEXT_DE",
        Language::English => "INFOTEXT_EN",
        Language::French => "INFOTEXT_FR",
        Language::Italian => "INFOTEXT_IT",
    };
    println!("Parsing {}...", filename);

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a InformationText instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 9, ExpectedType::Integer32),
            ColumnDefinition::new(11, -1, ExpectedType::String),
        ]),
    ]);
    let file_path = format!("data/{}", filename);
    let file_parser = FileParser::new(&file_path, row_parser)?;

    file_parser
        .parse()
        .for_each(|(_, _, values)| set_content(values, information_texts_primary_index, language));

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_information_texts_primary_index(
    information_texts: &InformationTextCollection,
) -> InformationTextPrimaryIndex {
    information_texts
        .iter()
        .fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.id(), Rc::clone(item));
            acc
        })
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn create_information_text(mut values: Vec<ParsedValue>) -> Rc<InformationText> {
    let id: i32 = values.remove(0).into();

    Rc::new(InformationText::new(id))
}

fn set_content(
    mut values: Vec<ParsedValue>,
    information_texts_primary_index: &InformationTextPrimaryIndex,
    language: Language,
) {
    let id: i32 = values.remove(0).into();
    let description: String = values.remove(0).into();

    information_texts_primary_index
        .get(&id)
        .unwrap()
        .set_content(language, &description);
}
