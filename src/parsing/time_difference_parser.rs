// 1 file(s).
// File(s) read by the parser:
// ZEITVS
use std::{error::Error, rc::Rc};

use chrono::NaiveDateTime;

use crate::{
    models::TimeDifference,
    parsing::{
        ColumnDefinition, ExpectedType, FastRowMatcher, FileParser, RowDefinition, RowParser,
    },
    storage::TimeDifferenceData,
};

use super::ParsedValue;

pub fn parse() -> Result<TimeDifferenceData, Box<dyn Error>> {
    println!("Parsing ZEITVS...");
    const ROW_A: i32 = 1;
    const ROW_B: i32 = 2;
    const ROW_C: i32 = 3;
    const ROW_D: i32 = 4;

    #[rustfmt::skip]
    let row_parser = RowParser::new(vec![
        // This row is ignored.
        RowDefinition::new(ROW_A, Box::new(FastRowMatcher::new(1, 1, "%", true)), Vec::new()),
        // This row is used to create a TimeDifference instance.
        RowDefinition::new(ROW_B, Box::new(FastRowMatcher::new(15, 1, "+", true)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 13, ExpectedType::Integer32),
            // Year 1:
            ColumnDefinition::new(15, 19, ExpectedType::Integer32),
            ColumnDefinition::new(21, 28, ExpectedType::Integer32),
            ColumnDefinition::new(30, 33, ExpectedType::Integer16),
            ColumnDefinition::new(35, 42, ExpectedType::Integer32),
            ColumnDefinition::new(44, 47, ExpectedType::Integer16),
            // Year 2:
            ColumnDefinition::new(49, 53, ExpectedType::Integer32),
            ColumnDefinition::new(55, 62, ExpectedType::Integer32),
            ColumnDefinition::new(64, 67, ExpectedType::Integer16),
            ColumnDefinition::new(69, 76, ExpectedType::Integer32),
            ColumnDefinition::new(78, 81, ExpectedType::Integer16),
        ]),
        // This row is used to create a TimeDifference instance.
        RowDefinition::new(ROW_C, Box::new(FastRowMatcher::new(15, 1, "+", false)), vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 13, ExpectedType::Integer32),
        ]),
    ]);
    let file_parser = FileParser::new("data/ZEITVS", row_parser)?;

    let mut rows = Vec::new();

    for (id, _, values) in file_parser.parse() {
        match id {
            ROW_A | ROW_D => continue,
            ROW_B => rows.push(create_instance_full(values)?),
            ROW_C => rows.push(create_instance(values)),
            _ => unreachable!(),
        }
    }

    Ok(TimeDifferenceData::new(rows))
}

// ------------------------------------------------------------------------------------------------
// --- Data Processing Functions
// ------------------------------------------------------------------------------------------------

fn create_instance_full(
    mut values: Vec<ParsedValue>,
) -> Result<Rc<TimeDifference>, Box<dyn Error>> {
    let stop_id: i32 = values.remove(0).into();
    let time_zone: i32 = values.remove(0).into();

    let time_zone_summer_1: i32 = values.remove(0).into();
    let start_date_1: i32 = values.remove(0).into();
    let start_hour_1: i16 = values.remove(0).into();
    let end_date_1: i32 = values.remove(0).into();
    let end_hour_1: i16 = values.remove(0).into();

    let start_date_1 = NaiveDateTime::parse_from_str(
        &format!("{:08}{:04}", start_date_1, start_hour_1),
        "%d%m%Y%H%M",
    )?;
    let end_date_1 = NaiveDateTime::parse_from_str(
        &format!("{:08}{:04}", end_date_1, end_hour_1),
        "%d%m%Y%H%M",
    )?;

    let time_zone_summer_2: i32 = values.remove(0).into();
    let start_date_2: i32 = values.remove(0).into();
    let start_hour_2: i16 = values.remove(0).into();
    let end_date_2: i32 = values.remove(0).into();
    let end_hour_2: i16 = values.remove(0).into();

    let start_date_2 = NaiveDateTime::parse_from_str(
        &format!("{:08}{:04}", start_date_2, start_hour_2),
        "%d%m%Y%H%M",
    )?;
    let end_date_2 = NaiveDateTime::parse_from_str(
        &format!("{:08}{:04}", end_date_2, end_hour_2),
        "%d%m%Y%H%M",
    )?;

    Ok(Rc::new(TimeDifference::new(
        stop_id,
        time_zone,
        Some(time_zone_summer_1),
        Some(start_date_1),
        Some(end_date_1),
        Some(time_zone_summer_2),
        Some(start_date_2),
        Some(end_date_2),
    )))
}

fn create_instance(mut values: Vec<ParsedValue>) -> Rc<TimeDifference> {
    let stop_id: i32 = values.remove(0).into();
    let time_zone: i32 = values.remove(0).into();

    Rc::new(TimeDifference::new(
        stop_id, time_zone, None, None, None, None, None, None,
    ))
}
