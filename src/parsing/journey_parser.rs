// 1 file(s).
// File(s) read by the parser:
// FPLAN
use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{
        Attribute, AutoIncrement, Direction, Journey, JourneyMetadataEntry, JourneyMetadataType,
        JourneyRouteEntry, Model, ResourceIndex, TransportType,
    },
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, RowDefinition, RowParser,
    },
    storage::SimpleResourceStorage,
};

use super::ParsedValue;

pub fn parse(
    transport_type_legacy_pk_index: &ResourceIndex<TransportType, String>,
    attribute_legacy_pk_index: &ResourceIndex<Attribute, String>,
    direction_legacy_pk_index: &ResourceIndex<Direction, String>,
) -> Result<
    (
        SimpleResourceStorage<Journey>,
        ResourceIndex<Journey, (i32, String)>,
    ),
    Box<dyn Error>,
> {
    println!("Parsing FPLAN...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;
    const ROW_E: i32 = 5;
    const ROW_F: i32 = 6;
    const ROW_G: i32 = 7;
    const ROW_H: i32 = 8;
    const ROW_I: i32 = 9;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is used to create a Journey instance.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(1, 2, "*Z", true)), vec![
            ColumnDefinition::new(4, 9, ExpectedType::Integer32),
            ColumnDefinition::new(11, 16, ExpectedType::String),
        ]),
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(1, 2, "*G", true)), vec![
            ColumnDefinition::new(4, 6, ExpectedType::String),
            ColumnDefinition::new(8, 14, ExpectedType::OptionInteger32),
            ColumnDefinition::new(16, 22, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(1, 5, "*A VE", true)), vec![
            ColumnDefinition::new(7, 13, ExpectedType::OptionInteger32),
            ColumnDefinition::new(15, 21, ExpectedType::OptionInteger32),
            ColumnDefinition::new(23, 28, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_D, Box::new(FastRowMatcher::new(1, 2, "*A", true)), vec![
            ColumnDefinition::new(4, 5, ExpectedType::String),
            ColumnDefinition::new(7, 13, ExpectedType::OptionInteger32),
            ColumnDefinition::new(15, 21, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_E, Box::new(FastRowMatcher::new(1, 2, "*I", true)), vec![
            ColumnDefinition::new(4, 5, ExpectedType::String),
            ColumnDefinition::new(7, 13, ExpectedType::OptionInteger32),
            ColumnDefinition::new(15, 21, ExpectedType::OptionInteger32),
            ColumnDefinition::new(23, 28, ExpectedType::OptionInteger32),
            ColumnDefinition::new(30, 38, ExpectedType::Integer32),
            ColumnDefinition::new(40, 45, ExpectedType::OptionInteger32),
            ColumnDefinition::new(47, 52, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_F, Box::new(FastRowMatcher::new(1, 2, "*L", true)), vec![
            ColumnDefinition::new(4, 11, ExpectedType::String),
            ColumnDefinition::new(13, 19, ExpectedType::OptionInteger32),
            ColumnDefinition::new(21, 27, ExpectedType::OptionInteger32),
            ColumnDefinition::new(29, 34, ExpectedType::OptionInteger32),
            ColumnDefinition::new(36, 41, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_G, Box::new(FastRowMatcher::new(1, 2, "*R", true)), vec![
            ColumnDefinition::new(4, 4, ExpectedType::String),
            ColumnDefinition::new(6, 12, ExpectedType::String),
            ColumnDefinition::new(14, 20, ExpectedType::OptionInteger32),
            ColumnDefinition::new(22, 28, ExpectedType::OptionInteger32),
            ColumnDefinition::new(30, 35, ExpectedType::OptionInteger32),
            ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
        ]),
        // *CI
        RowDefinition::new(ROW_H, Box::new(FastRowMatcher::new(1, 2, "*CI", true)), vec![
            ColumnDefinition::new(1, 3, ExpectedType::String),
            ColumnDefinition::new(5, 8, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::OptionInteger32),
            ColumnDefinition::new(18, 24, ExpectedType::OptionInteger32),
        ]),
        // *CO
        RowDefinition::new(ROW_H, Box::new(FastRowMatcher::new(1, 2, "*CO", true)), vec![
            ColumnDefinition::new(1, 3, ExpectedType::String),
            ColumnDefinition::new(5, 8, ExpectedType::Integer32),
            ColumnDefinition::new(10, 16, ExpectedType::OptionInteger32),
            ColumnDefinition::new(18, 24, ExpectedType::OptionInteger32),
        ]),
        RowDefinition::new(ROW_I, Box::new(FastRowMatcher::new(1, 0, "", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(30, 35, ExpectedType::OptionInteger32),
            ColumnDefinition::new(37, 42, ExpectedType::OptionInteger32),
        ]),
    ]);
    let file_parser = FileParser::new("data/FPLAN", row_parser)?;

    let mut rows = Vec::new();
    let mut legacy_pk_index = HashMap::new();
    let mut current_instance = Rc::new(Journey::default());
    let auto_increment = AutoIncrement::new();

    for (id, _, values) in file_parser.parse() {
        match id {
            ROW_A => {
                if auto_increment.value() == 1 {
                    break;
                }

                let (instance, k) = create_instance(values, &auto_increment);
                legacy_pk_index.insert(k, Rc::clone(&instance));
                rows.push(Rc::clone(&instance));
                current_instance = instance;
            }
            ROW_B => set_transport_type(values, &current_instance, &transport_type_legacy_pk_index),
            ROW_C => set_bit_field(values, &current_instance),
            ROW_D => set_attribute(values, &current_instance, &attribute_legacy_pk_index),
            ROW_E => set_information_text(values, &current_instance),
            ROW_F => set_line(values, &current_instance),
            ROW_G => set_direction(values, &current_instance, direction_legacy_pk_index),
            ROW_H => set_boarding_or_disembarking_transfer_time(values, &current_instance),
            ROW_I => set_route(values, &current_instance),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", rows);
    Ok((SimpleResourceStorage::new(rows), legacy_pk_index))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance(
    mut values: Vec<ParsedValue>,
    auto_increment: &AutoIncrement,
) -> (Rc<Journey>, (i32, String)) {
    let legacy_id: i32 = values.remove(0).into();
    let administration: String = values.remove(0).into();

    let instance = Rc::new(Journey::new(
        auto_increment.next(),
        administration.to_owned(),
    ));
    (instance, (legacy_id, administration))
}

fn set_transport_type(
    mut values: Vec<ParsedValue>,
    journey: &Rc<Journey>,
    transport_type_legacy_pk_index: &ResourceIndex<TransportType, String>,
) {
    let designation: String = values.remove(0).into();
    let from_stop_id: Option<i32> = values.remove(0).into();
    let until_stop_id: Option<i32> = values.remove(0).into();

    let transport_type_id = transport_type_legacy_pk_index
        .get(&designation)
        .unwrap()
        .id();

    journey.add_metadata_entry(
        JourneyMetadataType::TransportType,
        JourneyMetadataEntry::new(
            from_stop_id,
            until_stop_id,
            Some(transport_type_id),
            None,
            None,
            None,
            None,
            None,
        ),
    );
}

fn set_bit_field(mut values: Vec<ParsedValue>, journey: &Rc<Journey>) {
    let from_stop_id: Option<i32> = values.remove(0).into();
    let until_stop_id: Option<i32> = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();

    journey.add_metadata_entry(
        JourneyMetadataType::BitField,
        JourneyMetadataEntry::new(
            from_stop_id,
            until_stop_id,
            None,
            bit_field_id,
            None,
            None,
            None,
            None,
        ),
    );
}

fn set_attribute(
    mut values: Vec<ParsedValue>,
    journey: &Rc<Journey>,
    attribute_legacy_pk_index: &ResourceIndex<Attribute, String>,
) {
    let designation: String = values.remove(0).into();
    let from_stop_id: Option<i32> = values.remove(0).into();
    let until_stop_id: Option<i32> = values.remove(0).into();

    let attribute_id = attribute_legacy_pk_index.get(&designation).unwrap().id();

    journey.add_metadata_entry(
        JourneyMetadataType::Attribute,
        JourneyMetadataEntry::new(
            from_stop_id,
            until_stop_id,
            Some(attribute_id),
            None,
            None,
            None,
            None,
            None,
        ),
    );
}

fn set_information_text(mut values: Vec<ParsedValue>, journey: &Rc<Journey>) {
    let code: String = values.remove(0).into();
    let from_stop_id: Option<i32> = values.remove(0).into();
    let until_stop_id: Option<i32> = values.remove(0).into();
    let bit_field_id: Option<i32> = values.remove(0).into();
    let information_text_id: i32 = values.remove(0).into();
    let departure_time: Option<i32> = values.remove(0).into();
    let arrival_time: Option<i32> = values.remove(0).into();

    journey.add_metadata_entry(
        JourneyMetadataType::InformationText,
        JourneyMetadataEntry::new(
            from_stop_id,
            until_stop_id,
            Some(information_text_id),
            bit_field_id,
            departure_time,
            arrival_time,
            Some(code),
            None,
        ),
    );
}

fn set_line(mut values: Vec<ParsedValue>, journey: &Rc<Journey>) {
    let line_designation: String = values.remove(0).into();
    let from_stop_id: Option<i32> = values.remove(0).into();
    let until_stop_id: Option<i32> = values.remove(0).into();
    let departure_time: i32 = values.remove(0).into();
    let arrival_time: i32 = values.remove(0).into();

    let (resource_id, extra_field1) = if line_designation.chars().next().unwrap() == '#' {
        (
            Some(line_designation[1..].parse::<i32>().unwrap()),
            None,
        )
    } else {
        (None, Some(line_designation))
    };

    journey.add_metadata_entry(
        JourneyMetadataType::Line,
        JourneyMetadataEntry::new(
            from_stop_id,
            until_stop_id,
            resource_id,
            None,
            Some(departure_time),
            Some(arrival_time),
            extra_field1,
            None,
        ),
    );
}

fn set_direction(
    mut values: Vec<ParsedValue>,
    journey: &Rc<Journey>,
    direction_legacy_pk_index: &ResourceIndex<Direction, String>,
) {
    let round_trip: String = values.remove(0).into();
    let direction_id: String = values.remove(0).into();
    let from_stop_id: Option<i32> = values.remove(0).into();
    let until_stop_id: Option<i32> = values.remove(0).into();
    let departure_time: Option<i32> = values.remove(0).into();
    let arrival_time: Option<i32> = values.remove(0).into();

    if direction_id.is_empty() {
        journey.add_metadata_entry(
            JourneyMetadataType::Direction,
            JourneyMetadataEntry::new(None, None, None, None, None, None, Some(round_trip), None),
        );
    } else {
        let direction_id = direction_legacy_pk_index.get(&direction_id).unwrap().id();

        journey.add_metadata_entry(
            JourneyMetadataType::Direction,
            JourneyMetadataEntry::new(
                from_stop_id,
                until_stop_id,
                Some(direction_id),
                None,
                departure_time,
                arrival_time,
                Some(round_trip),
                None,
            ),
        );
    }
}

fn set_boarding_or_disembarking_transfer_time(mut values: Vec<ParsedValue>, journey: &Rc<Journey>) {
    let ci_co: String = values.remove(0).into();
    let from_stop_id: Option<i32> = values.remove(0).into();
    let until_stop_id: Option<i32> = values.remove(0).into();
    let transfer_time: i32 = values.remove(0).into();

    let metadata_type = if ci_co == "*CI" {
        JourneyMetadataType::TransferTimeBoarding
    } else {
        JourneyMetadataType::TransferTimeDisembarking
    };

    journey.add_metadata_entry(
        metadata_type,
        JourneyMetadataEntry::new(
            from_stop_id,
            until_stop_id,
            None,
            None,
            None,
            None,
            None,
            Some(transfer_time),
        ),
    );
}

fn set_route(mut values: Vec<ParsedValue>, journey: &Rc<Journey>) {
    let stop_id: i32 = values.remove(0).into();
    let arrival_time: Option<i32> = values.remove(0).into();
    let departure_time: Option<i32> = values.remove(0).into();

    journey.add_route_entry(JourneyRouteEntry::new(
        stop_id,
        arrival_time,
        departure_time,
    ));
}
