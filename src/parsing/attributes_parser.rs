// File(s) read by the parser:
// ATTRIBUT => it's unclear whether the format matches the standard or not.
// ---
// Files not used by the parser:
// ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT
use std::{collections::HashMap, error::Error, rc::Rc, str::FromStr};

use crate::{
    models::{Attribute, AttributeCollection, AttributePrimaryIndex, Language},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, FileParser,
        RowDefinition, RowParser,
    },
};

use super::ParsedValue;

pub fn load_attributes() -> Result<(AttributeCollection, AttributePrimaryIndex), Box<dyn Error>> {
    println!("Parsing ATTRIBUT...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create an Attribute instance.
        RowDefinition::new(ROW_A, Box::new(
            AdvancedRowMatcher::new("^.{2} [0-9] [0-9 ]{3} [0-9 ]{2}$")?
        ), vec![
            ColumnDefinition::new(1, 2, ExpectedType::String),
            ColumnDefinition::new(4, 4, ExpectedType::Integer16),
            ColumnDefinition::new(6, 8, ExpectedType::Integer16),
            ColumnDefinition::new(10, 11, ExpectedType::Integer16),
        ]),
        // This row is ignored. TODO : should i take care of this row?, e.g. # 1  1  1
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 1, "#", true)), Vec::new()),
        // This row indicates the language of the description.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(1, 1, "<", true)), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String), // This column has not been explicitly defined in the standard.
        ]),
        // This row contains the description in a specific language.
        RowDefinition::new(ROW_D, Box::new(
            AdvancedRowMatcher::new("^.{2} .+$")?
        ), vec![
            ColumnDefinition::new(1, 2, ExpectedType::String),
            ColumnDefinition::new(4, -1, ExpectedType::String),
        ]),
    ]);
    // The ATTRIBUT file is used instead of ATTRIBUT_* for simplicity's sake.
    let file_parser = FileParser::new("data/ATTRIBUT", row_parser)?;

    let mut attributes = Vec::new();
    let mut attributes_primary_index = None;
    let mut current_language = Language::default();

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => attributes.push(create_attribute(values)),
        ROW_B => {
            if attributes_primary_index.is_none() {
                // When ROW_B is reached, all instances have already been created.
                // The primary index is then created only once.
                attributes_primary_index = Some(create_attributes_primary_index(&attributes));
            }
        }
        ROW_C => update_current_language(values, &mut current_language),
        ROW_D => set_description(values, &attributes_primary_index, current_language),
        _ => unreachable!(),
    });

    Ok((attributes, attributes_primary_index.unwrap()))
}

// ------------------------------------------------------------------------------------------------
// --- Indexes Creation
// ------------------------------------------------------------------------------------------------

fn create_attributes_primary_index(attributes: &AttributeCollection) -> AttributePrimaryIndex {
    attributes.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id().to_owned(), Rc::clone(item));
        acc
    })
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn create_attribute(mut values: Vec<ParsedValue>) -> Rc<Attribute> {
    let id: String = values.remove(0).into();
    let stop_scope: i16 = values.remove(0).into();
    let main_sorting_priority: i16 = values.remove(0).into();
    let secondary_sorting_priority: i16 = values.remove(0).into();

    Rc::new(Attribute::new(
        id,
        stop_scope,
        main_sorting_priority,
        secondary_sorting_priority,
    ))
}

fn update_current_language(mut values: Vec<ParsedValue>, current_language: &mut Language) {
    let language: String = values.remove(0).into();
    let language = &language[1..&language.len() - 1];

    if language != "text" {
        *current_language = Language::from_str(language).unwrap();
    }
}

fn set_description(
    mut values: Vec<ParsedValue>,
    attributes_primary_index: &Option<AttributePrimaryIndex>,
    language: Language,
) {
    let id: String = values.remove(0).into();
    let description: String = values.remove(0).into();

    attributes_primary_index
        .as_ref()
        .unwrap()
        .get(&id)
        .unwrap()
        .set_description(language, &description);
}
