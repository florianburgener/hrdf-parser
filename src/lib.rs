use std::{error::Error, cmp};

#[derive(Debug)]
enum ExpectedType {
    // Float,
    // Integer16,
    Integer32,
    String,
}

#[derive(Debug)]
enum ParsedValue {
    // Float(f64),
    // Integer16(i16),
    Integer32(i32),
    String(String),
}

#[derive(Debug)]
struct ColumnDefinition {
    start: usize,
    stop: usize,
    expected_type: ExpectedType,
}

impl ColumnDefinition {
    fn new(start: usize, stop: usize, expected_type: ExpectedType) -> Self {
        ColumnDefinition {
            start,
            stop,
            expected_type,
        }
    }
}

fn parse_row(row_configuration: Vec<ColumnDefinition>, raw_row: &str) -> Vec<ParsedValue> {
    let mut values = vec![];

    for column_definition in &row_configuration {
        let start = column_definition.start - 1;
        let stop = cmp::min(column_definition.stop, raw_row.len());
        let value = &raw_row[start..stop];

        let value = match column_definition.expected_type {
            // ExpectedType::Float => ParsedValue::Float(value.parse::<f64>().unwrap()),
            // ExpectedType::Integer16 => ParsedValue::Integer16(value.parse::<i16>().unwrap()),
            ExpectedType::Integer32 => ParsedValue::Integer32(value.parse::<i32>().unwrap()),
            ExpectedType::String => ParsedValue::String(value.parse::<String>().unwrap()),
        };

        values.push(value);
    }

    values
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(13, 62, ExpectedType::String),
    ];
    let raw_row = "8507000     Bern$<1>$BN$<3>";

    let values = parse_row(row_configuration, raw_row);

    for value in &values {
        println!("{:?}", value);
    }

    Ok(())
}
