// 1 file(s).
// File(s) read by the parser:
// UMSTEIGL
use std::{error::Error, rc::Rc};

use crate::{
    models::{AutoIncrement, LineTransferTime, Model, ResourceIndex, TransportType},
    parsing::{ColumnDefinition, ExpectedType, FileParser, RowDefinition, RowParser},
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse(
    transport_types_legacy_pk_index: &ResourceIndex<TransportType, String>,
) -> Result<SimpleResourceStorage<LineTransferTime>, Box<dyn Error>> {
    println!("Parsing UMSTEIGL...");
    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a LineTransferTime instance.
        RowDefinition::from(vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 14, ExpectedType::String),
            ColumnDefinition::new(16, 18, ExpectedType::String),
            ColumnDefinition::new(20, 27, ExpectedType::String),
            ColumnDefinition::new(29, 29, ExpectedType::String),
            ColumnDefinition::new(31, 36, ExpectedType::String),
            ColumnDefinition::new(38, 40, ExpectedType::String),
            ColumnDefinition::new(42, 49, ExpectedType::String),
            ColumnDefinition::new(51, 51, ExpectedType::String),
            ColumnDefinition::new(53, 55, ExpectedType::Integer16),
            ColumnDefinition::new(56, 56, ExpectedType::String),
        ]),
    ]);
    let parser = FileParser::new("data/UMSTEIGL", row_parser)?;

    let auto_increment = AutoIncrement::new();

    let rows = parser
        .parse()
        .filter_map(|(_, _, values)| {
            Some(create_instance(
                values,
                &auto_increment,
                transport_types_legacy_pk_index,
            ))
        })
        .collect();

    Ok(SimpleResourceStorage::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
    transport_types_legacy_pk_index: &ResourceIndex<TransportType, String>,
) -> Rc<LineTransferTime> {
    let stop_id: i32 = values.remove(0).into();
    let administration_1: String = values.remove(0).into();
    let transport_type_id_1: String = values.remove(0).into();
    let line_id_1: String = values.remove(0).into();
    let direction_1: String = values.remove(0).into();
    let administration_2: String = values.remove(0).into();
    let transport_type_id_2: String = values.remove(0).into();
    let line_id_2: String = values.remove(0).into();
    let direction_2: String = values.remove(0).into();
    let duration: i16 = values.remove(0).into();
    let is_guaranteed: String = values.remove(0).into();

    let transport_type_id_1 = transport_types_legacy_pk_index
        .get(&transport_type_id_1)
        .unwrap()
        .id();

    let line_id_1 = if line_id_1 == "*" {
        None
    } else {
        Some(line_id_1)
    };

    let direction_1 = if direction_1 == "*" {
        None
    } else {
        Some(direction_1)
    };

    let transport_type_id_2 = transport_types_legacy_pk_index
        .get(&transport_type_id_2)
        .unwrap()
        .id();

    let line_id_2 = if line_id_2 == "*" {
        None
    } else {
        Some(line_id_2)
    };

    let direction_2 = if direction_2 == "*" {
        None
    } else {
        Some(direction_2)
    };

    let is_guaranteed = is_guaranteed == "!";

    Rc::new(LineTransferTime::new(
        auto_increment.next(),
        stop_id,
        administration_1,
        transport_type_id_1,
        line_id_1,
        direction_1,
        administration_2,
        transport_type_id_2,
        line_id_2,
        direction_2,
        duration,
        is_guaranteed,
    ))
}
