// 1 file(s).
// File(s) read by the parser:
// ZUGART
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Language, ResourceCollection, ResourceIndex, TransportType},
    parsing::{
        AdvancedRowMatcher, ColumnDefinition, ExpectedType, FastRowMatcher, RowDefinition,
        RowParser,
    },
    storage::SimpleResourceStorage,
};

use super::{FileParser, ParsedValue};

pub fn parse() -> Result<(SimpleResourceStorage<TransportType>, ResourceIndex<TransportType, String>), Box<dyn Error>> {
    println!("Parsing ZUGART...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;
    const ROW_E: i32 = 5;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a TransportType instance.
        RowDefinition::new(ROW_A, Box::new(
            AdvancedRowMatcher::new(r"^.{3} [ 0-9]{2}")?
        ), vec![
            ColumnDefinition::new(1, 3, ExpectedType::String),
            ColumnDefinition::new(5, 6, ExpectedType::Integer16),
            ColumnDefinition::new(8, 8, ExpectedType::String),
            ColumnDefinition::new(10, 10, ExpectedType::Integer16),
            ColumnDefinition::new(12, 19, ExpectedType::String),
            ColumnDefinition::new(21, 21, ExpectedType::Integer16),
            ColumnDefinition::new(23, 23, ExpectedType::String),
        ]),
        // This row indicates the language for translations in the section that follows it.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 1, "<", true)), vec![
            ColumnDefinition::new(1, -1, ExpectedType::String),
        ]),
        // This row contains the product class name in a specific language.
        RowDefinition::new(ROW_C, Box::new(
            AdvancedRowMatcher::new(r"^class.+$")?
        ), vec![
            ColumnDefinition::new(6, 7, ExpectedType::Integer16),
            ColumnDefinition::new(9, -1, ExpectedType::String),
        ]),
        // This row is ignored.
        RowDefinition::new(ROW_D, Box::new(AdvancedRowMatcher::new(r"^option.+$")?), Vec::new()),
        // This row contains the category name in a specific language.
        RowDefinition::new(ROW_E, Box::new(
            AdvancedRowMatcher::new(r"^category.+$")?
        ), vec![
            ColumnDefinition::new(10, 12, ExpectedType::Integer32),
            ColumnDefinition::new(14, -1, ExpectedType::String),
        ]),
    ]);
    let file_parser = FileParser::new("data/ZUGART", row_parser)?;

    let mut rows = Vec::new();
    let mut legacy_primary_index = HashMap::new();

    let mut next_id = 1;
    let mut current_language = Language::default();

    file_parser.parse().for_each(|(id, _, values)| match id {
        ROW_A => {
            let (instance, k) = create_instance(values, next_id);
            legacy_primary_index.insert(k, Rc::clone(&instance));
            rows.push(instance);
            next_id += 1;
        }
        ROW_B => update_current_language(values, &mut current_language),
        ROW_C => set_product_class_name(values, &rows, current_language),
        ROW_D => return,
        ROW_E => set_category_name(values, &rows, current_language),
        _ => unreachable!(),
    });

    Ok((SimpleResourceStorage::new(rows), legacy_primary_index))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(mut values: Vec<ParsedValue>, id: i32) -> (Rc<TransportType>, String) {
    let designation: String = values.remove(0).into();
    let product_class_id: i16 = values.remove(0).into();
    let tarrif_group: String = values.remove(0).into();
    let output_control: i16 = values.remove(0).into();
    let short_name: String = values.remove(0).into();
    let surchage: i16 = values.remove(0).into();
    let flag: String = values.remove(0).into();

    let instance = Rc::new(TransportType::new(
        id,
        designation.to_owned(),
        product_class_id,
        tarrif_group,
        output_control,
        short_name,
        surchage,
        flag,
    ));
    (instance, designation)
}

fn set_product_class_name(
    mut values: Vec<ParsedValue>,
    rows: &ResourceCollection<TransportType>,
    language: Language,
) {
    let id: i16 = values.remove(0).into();
    let product_class_name: String = values.remove(0).into();

    for row in rows.iter() {
        if row.product_class_id() == id {
            row.set_product_class_name(language, &product_class_name);
        }
    }
}

fn set_category_name(
    mut values: Vec<ParsedValue>,
    rows: &ResourceCollection<TransportType>,
    language: Language,
) {
    let index: i32 = values.remove(0).into();
    let index = index as usize;
    let category_name: String = values.remove(0).into();

    rows.get(index - 1)
        .unwrap()
        .set_category_name(language, &category_name);
}

// ------------------------------------------------------------------------------------------------
// --- Helper Functions
// ------------------------------------------------------------------------------------------------

fn update_current_language(mut values: Vec<ParsedValue>, current_language: &mut Language) {
    let language: String = values.remove(0).into();
    let language = &language[1..&language.len() - 1];

    if language != "text" {
        *current_language = match language {
            "Deutsch" => Language::German,
            "Franzoesisch" => Language::French,
            "Englisch" => Language::English,
            "Italienisch" => Language::Italian,
            _ => unreachable!(),
        };
    }
}
