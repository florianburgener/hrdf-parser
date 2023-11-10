use std::{
    cmp,
    collections::HashMap,
    fs::{self},
    io,
};

pub enum ExpectedType {
    Float,
    Integer16,
    Integer32,
    String,
}

#[derive(Debug)]
pub enum ParsedValue {
    Float(f64),
    Integer16(i16),
    Integer32(i32),
    String(String),
}

impl From<ParsedValue> for f64 {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::Float(x) => x,
            _ => panic!("Failed to convert ParsedValue to f64"),
        }
    }
}

impl From<ParsedValue> for i16 {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::Integer16(x) => x,
            _ => panic!("Failed to convert ParsedValue to i16"),
        }
    }
}

impl From<ParsedValue> for i32 {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::Integer32(x) => x,
            _ => panic!("Failed to convert ParsedValue to i32"),
        }
    }
}

impl From<ParsedValue> for String {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::String(x) => x,
            _ => panic!("Failed to convert ParsedValue to String"),
        }
    }
}

pub struct ColumnDefinition {
    start: usize,
    stop: isize,
    expected_type: ExpectedType,
}

impl ColumnDefinition {
    pub fn new(start: usize, stop: isize, expected_type: ExpectedType) -> Self {
        Self {
            start,
            stop,
            expected_type,
        }
    }
}

type RowConfiguration = Vec<ColumnDefinition>;

pub trait RowParser {
    fn parse(&self, row: &str) -> Vec<ParsedValue> {
        let values = self.row_configuration(row)
            .iter()
            .map(|column_definition| {
                let start = column_definition.start - 1;
                let stop;

                if column_definition.stop == -1 {
                    stop = row.len()
                } else {
                    stop = cmp::min(column_definition.stop as usize, row.len());
                }

                let value = row[start..stop].trim();

                match column_definition.expected_type {
                    ExpectedType::Float => ParsedValue::Float(value.parse::<f64>().unwrap()),
                    ExpectedType::Integer16 => {
                        ParsedValue::Integer16(value.parse::<i16>().unwrap())
                    }
                    ExpectedType::Integer32 => {
                        ParsedValue::Integer32(value.parse::<i32>().unwrap())
                    }
                    ExpectedType::String => ParsedValue::String(value.parse::<String>().unwrap()),
                }
            })
            .collect();
        values
    }

    fn row_configuration(&self, row: &str) -> &RowConfiguration;
}

pub struct SingleConfigurationRowParser {
    row_configuration: RowConfiguration,
}

impl SingleConfigurationRowParser {
    pub fn new(row_configuration: RowConfiguration) -> Self {
        Self { row_configuration }
    }
}

impl RowParser for SingleConfigurationRowParser {
    fn row_configuration(&self, _: &str) -> &RowConfiguration {
        &self.row_configuration
    }
}

// MultipleConfigurationRowParser
// SequentialConfigurationRowParser

pub struct FileParser {
    rows: Vec<String>,
    row_parser: Box<dyn RowParser>,
}

impl FileParser {
    pub fn new(file_path: &str, row_parser: Box<dyn RowParser>) -> io::Result<Self> {
        let contents = Self::read_file(file_path)?;
        let rows = contents.lines().map(String::from).collect();

        Ok(Self { rows, row_parser })
    }

    fn read_file(file_path: &str) -> io::Result<String> {
        fs::read_to_string(file_path)
    }

    pub fn iter(&self) -> FileParserIterator {
        FileParserIterator {
            rows_iter: self.rows.iter(),
            row_parser: &self.row_parser,
        }
    }
}

// Iterator implementation for FileParser

pub struct FileParserIterator<'a> {
    rows_iter: std::slice::Iter<'a, String>,
    row_parser: &'a Box<dyn RowParser>,
}

impl Iterator for FileParserIterator<'_> {
    type Item = Vec<ParsedValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows_iter.next().map(|row| self.row_parser.parse(row))
    }
}

pub fn parse_stop_name(name: String) -> HashMap<i32, Vec<String>> {
    let parsed_name: HashMap<i32, Vec<String>> = name
        .split('>')
        .filter(|&s| !s.is_empty())
        .map(|s| s.replace('$', ""))
        .map(|s| {
            let mut parts = s.split('<');

            let value = parts.next().unwrap().to_string();
            let key = parts.next().unwrap().parse::<i32>().unwrap();

            (key, value)
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert(Vec::new()).push(value);
            acc
        });
    parsed_name
}
