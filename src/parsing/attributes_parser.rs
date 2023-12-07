// ATTRIBUT
// Unused files: ATTRIBUT_DE, ATTRIBUT_EN, ATTRIBUT_FR, ATTRIBUT_IT
use std::{collections::HashMap, error::Error, rc::Rc, str::FromStr};

use regex::Regex;

use crate::{
    models::{Attribute, Language},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowMatcher, RowParser},
};

pub fn load_attributes(
) -> Result<(Vec<Rc<Attribute>>, HashMap<String, Rc<Attribute>>), Box<dyn Error>> {
    println!("Parsing ATTRIBUT...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    // TODO : "Complies with the standard."
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        RowDefinition::new(ROW_A, RowMatcher::new_with_re_only(
            Regex::new("^.{2} [0-9] [0-9 ]{3} [0-9 ]{2}$").unwrap()
        ), vec![
            ColumnDefinition::new(1, 2, ExpectedType::String),      // Complies with the standard.
            ColumnDefinition::new(4, 4, ExpectedType::Integer16),   // Complies with the standard.
            ColumnDefinition::new(6, 8, ExpectedType::Integer16),   // Complies with the standard.
            ColumnDefinition::new(10, 11, ExpectedType::Integer16), // Complies with the standard.
        ]),
        RowDefinition::new(ROW_B, RowMatcher::new(1, 1, "#", true), vec![]),
        RowDefinition::new(ROW_C, RowMatcher::new(1, 1, "<", true), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String), // Complies with the standard.
        ]),
        RowDefinition::new(ROW_D, RowMatcher::new_with_re_only(
            Regex::new("^.{2} .+$").unwrap()
        ), vec![
            ColumnDefinition::new(1, 2, ExpectedType::String),  // Complies with the standard.
            ColumnDefinition::new(4, -1, ExpectedType::String), // Complies with the standard.
        ]),
    ]);
    let file_parser = FileParser::new("data/ATTRIBUT", row_parser)?;

    let mut attributes = vec![];
    let mut attributes_primary_index = None;
    let mut current_language = Language::default();

    for (id, _, mut values) in file_parser.parse() {
        match id {
            ROW_A => {
                let id: String = values.remove(0).into();
                let stop_scope: i16 = values.remove(0).into();
                let main_sorting_priority: i16 = values.remove(0).into();
                let secondary_sorting_priority: i16 = values.remove(0).into();

                attributes.push(Rc::new(Attribute::new(
                    id,
                    stop_scope,
                    main_sorting_priority,
                    secondary_sorting_priority,
                )));
            }
            ROW_B => {
                if attributes_primary_index.is_none() {
                    // When ROW_B is reached, all instances have already been created.
                    // The primary index is then created only once.
                    attributes_primary_index = Some(create_attributes_primary_index(&attributes));
                }

                // TODO
            }
            ROW_C => {
                let language: String = values.remove(0).into();
                let language = &language[1..&language.len() - 1];

                if language != "text" {
                    current_language = Language::from_str(language).unwrap();
                }
            }
            ROW_D => {
                let id: String = values.remove(0).into();
                let description: String = values.remove(0).into();

                attributes_primary_index
                    .as_ref()
                    .unwrap()
                    .get(&id)
                    .unwrap()
                    .set_description(current_language, &description);
            }
            _ => unreachable!(),
        }
    }

    Ok((attributes, attributes_primary_index.unwrap()))
}

fn create_attributes_primary_index(
    attributes: &Vec<Rc<Attribute>>,
) -> HashMap<String, Rc<Attribute>> {
    attributes.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id().clone(), Rc::clone(item));
        acc
    })
}
