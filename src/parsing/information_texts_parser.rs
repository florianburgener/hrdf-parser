// File(s) read by the parser:
// INFOTEXT_DE => Format matches the standard.
// INFOTEXT_EN => Format matches the standard.
// INFOTEXT_FR => Format matches the standard.
// INFOTEXT_IT => Format matches the standard.
use std::{collections::HashMap, error::Error, rc::Rc, borrow::Borrow};

use crate::{
    models::{
        Direction, DirectionCollection, DirectionPrimaryIndex, InformationText,
        InformationTextCollection, Language,
    },
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser, information_texts_parser},
};

use super::ParsedValue;

pub fn load_information_texts() {
    let mut information_texts: InformationTextCollection = Vec::new();

    _load_information_texts(&mut information_texts, Language::German);
    _load_information_texts(&mut information_texts, Language::English);
    _load_information_texts(&mut information_texts, Language::French);
    _load_information_texts(&mut information_texts, Language::Italian);
    // let directions_primary_index = create_directions_primary_index(&directions);

    // Ok((directions, directions_primary_index))
}

fn _load_information_texts(
    information_texts: &mut InformationTextCollection,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    const ROW_A: i32 = 1;

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
        .for_each(|(_, _, mut values)| match language {
            Language::German => information_texts.push(create_information_text(values)),
            _ => {
                let id: i32 = values.remove(0).into();
                let content: String = values.remove(0).into();

                // unreachable!()
                for information_text in information_texts.iter() {
                    if information_text.id() == id {
                        information_text.set_content(language, &content);
                        break;
                    }
                }
            }
        });

    Ok(())
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_directions_primary_index(directions: &DirectionCollection) -> DirectionPrimaryIndex {
    directions.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id().to_owned(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn create_information_text(mut values: Vec<ParsedValue>) -> Rc<InformationText> {
    let id: i32 = values.remove(0).into();
    let content: String = values.remove(0).into();

    // TODO
    let mut d = HashMap::new();
    // The first description is always in German.
    d.insert(Language::German.to_string(), content);

    Rc::new(InformationText::new(id, d))
}
