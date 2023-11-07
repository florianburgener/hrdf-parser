use std::{
    cmp,
    collections::HashMap,
    fs::{self},
    io,
};

pub enum ExpectedType {
    // Float,
    // Integer16,
    Integer32,
    String,
}

#[derive(Debug)]
pub enum ParsedValue {
    // Float(f64),
    // Integer16(i16),
    Integer32(i32),
    String(String),
}

impl From<ParsedValue> for i32 {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::Integer32(x) => x,
            _ => panic!("Failed to convert ParsedValue to i32"),
        }
    }
}

pub struct ColumnDefinition {
    start: usize,
    stop: usize,
    expected_type: ExpectedType,
}

impl ColumnDefinition {
    pub fn new(start: usize, stop: usize, expected_type: ExpectedType) -> Self {
        Self {
            start,
            stop,
            expected_type,
        }
    }
}

type RowConfiguration = Vec<ColumnDefinition>;

pub trait RowParser {
    fn reset(&self) {}

    fn parse(&self, row: &str) -> Vec<ParsedValue> {
        let mut values = vec![];

        for column_definition in self.row_configuration() {
            let start = column_definition.start - 1;
            let stop = cmp::min(column_definition.stop, row.len());
            let value = &row[start..stop];

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

    fn row_configuration(&self) -> &RowConfiguration;
}

pub struct DefaultRowParser {
    row_configuration: RowConfiguration,
}

impl DefaultRowParser {
    pub fn new(row_configuration: RowConfiguration) -> Self {
        Self { row_configuration }
    }
}

impl RowParser for DefaultRowParser {
    fn row_configuration(&self) -> &RowConfiguration {
        &self.row_configuration
    }
}

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
            file_parser: self,
            index: 0,
        }
    }
}

// Iterator implementation for FileParser

pub struct FileParserIterator<'a> {
    file_parser: &'a FileParser,
    index: usize,
}

impl Iterator for FileParserIterator<'_> {
    type Item = Vec<ParsedValue>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.file_parser.rows.len() {
            let row = &self.file_parser.rows[self.index];
            let parsed_row = Some(self.file_parser.row_parser.parse(row));
            self.index += 1;
            parsed_row
        } else {
            None
        }
    }
}

// IntoIterator implementation for FileParser

pub struct FileParserIntoIterator {
    file_parser: FileParser,
}

impl Iterator for FileParserIntoIterator {
    type Item = Vec<ParsedValue>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.file_parser.rows.len() == 0 {
            None
        } else {
            let row = self.file_parser.rows.remove(0);
            Some(self.file_parser.row_parser.parse(&row))
        }
    }
}

impl IntoIterator for FileParser {
    type Item = Vec<ParsedValue>;
    type IntoIter = FileParserIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        FileParserIntoIterator { file_parser: self }
    }
}

pub fn parse_inline_values(item: &str) -> HashMap<i32, Vec<String>> {
    let inline_values: HashMap<i32, Vec<String>> = item
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
    inline_values
}
