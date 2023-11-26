mod platforms_parser;
mod stops_parser;
mod timetable_key_data_parser;

pub use platforms_parser::load_journey_platform_and_platforms;
pub use stops_parser::load_stops;
pub use timetable_key_data_parser::load_timetable_key_data;

use std::{
    fs::File,
    io::{self, Read, Seek},
};

pub enum ExpectedType {
    Float,
    Integer16,
    Integer32,
    String,
    OptionInteger16,
    OptionInteger32,
}

#[derive(Debug)]
pub enum ParsedValue {
    Float(f64),
    Integer16(i16),
    Integer32(i32),
    String(String),
    OptionInteger16(Option<i16>),
    OptionInteger32(Option<i32>),
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

impl From<ParsedValue> for Option<i16> {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::OptionInteger16(x) => x,
            _ => panic!("Failed to convert ParsedValue to Option<i16>"),
        }
    }
}

impl From<ParsedValue> for Option<i32> {
    fn from(value: ParsedValue) -> Self {
        match value {
            ParsedValue::OptionInteger32(x) => x,
            _ => panic!("Failed to convert ParsedValue to Option<i32>"),
        }
    }
}

pub struct RowMatcher {
    // 1-based indexing
    start: usize,
    length: usize,
    value: String,
    should_equal_value: bool,
}

impl RowMatcher {
    pub fn new(start: usize, length: usize, value: &str, should_equal_value: bool) -> RowMatcher {
        Self {
            start,
            length,
            value: value.to_string(),
            should_equal_value,
        }
    }

    fn match_row(&self, row: &str) -> bool {
        let start = self.start - 1;
        let target_value = &row[start..(start + self.length)];
        self.should_equal_value == (target_value == self.value)
    }
}

pub struct ColumnDefinition {
    // 1-based indexing
    start: usize,
    // 1-based indexing
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

pub struct RowDefinition {
    id: i32,
    row_matcher: Option<RowMatcher>,
    row_configuration: RowConfiguration,
}

impl RowDefinition {
    pub fn new(id: i32, row_matcher: RowMatcher, row_configuration: RowConfiguration) -> Self {
        Self {
            id,
            row_matcher: Some(row_matcher),
            row_configuration,
        }
    }
}

impl From<RowConfiguration> for RowDefinition {
    fn from(row_configuration: RowConfiguration) -> Self {
        Self {
            id: 1,
            row_matcher: None,
            row_configuration,
        }
    }
}

// (RowDefinition.id, current cursor position in the file,  parsed row values)
type ParsedRow = (i32, usize, Vec<ParsedValue>);

pub struct RowParser {
    row_definitions: Vec<RowDefinition>,
}

impl RowParser {
    pub fn new(row_definitions: Vec<RowDefinition>) -> Self {
        Self { row_definitions }
    }

    fn parse(&self, row: &str) -> ParsedRow {
        let row_definition = self.row_definition(row);
        // 2 bytes for \r\n
        let bytes_read = row.len() + 2;
        let values = row_definition
            .row_configuration
            .iter()
            .map(|column_definition| {
                let start = column_definition.start - 1;
                let stop = if column_definition.stop == -1 {
                    row.len()
                } else {
                    column_definition.stop as usize
                };

                let value = row[start..stop].trim();

                match column_definition.expected_type {
                    ExpectedType::Float => ParsedValue::Float(value.parse().unwrap()),
                    ExpectedType::Integer16 => ParsedValue::Integer16(value.parse().unwrap()),
                    ExpectedType::Integer32 => ParsedValue::Integer32(value.parse().unwrap()),
                    ExpectedType::String => ParsedValue::String(value.parse().unwrap()),
                    ExpectedType::OptionInteger16 => {
                        ParsedValue::OptionInteger16(value.parse().ok())
                    }
                    ExpectedType::OptionInteger32 => {
                        ParsedValue::OptionInteger32(value.parse().ok())
                    }
                }
            })
            .collect();
        (row_definition.id, bytes_read, values)
    }

    fn row_definition(&self, row: &str) -> &RowDefinition {
        if self.row_definitions.len() == 1 {
            return &self.row_definitions[0];
        }

        for row_definition in &self.row_definitions {
            if row_definition.row_matcher.as_ref().unwrap().match_row(row) {
                return row_definition;
            }
        }

        panic!("This type of row is unknown. The unknown row :\n{}", row);
    }
}

pub struct FileParser {
    rows: Vec<String>,
    row_parser: RowParser,
}

impl FileParser {
    pub fn new(path: &str, row_parser: RowParser) -> io::Result<Self> {
        Self::new_with_bytes_offset(path, row_parser, 0)
    }

    pub fn new_with_bytes_offset(
        path: &str,
        row_parser: RowParser,
        bytes_offset: u64,
    ) -> io::Result<Self> {
        let rows = Self::read_lines(path, bytes_offset)?;
        Ok(Self { rows, row_parser })
    }

    fn read_lines(path: &str, bytes_offset: u64) -> io::Result<Vec<String>> {
        let mut file = File::open(path)?;
        file.seek(io::SeekFrom::Start(bytes_offset))?;
        let mut reader = io::BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let lines = contents.lines().map(String::from).collect();
        Ok(lines)
    }

    pub fn parse(&self) -> ParsedRowIterator {
        ParsedRowIterator {
            rows_iter: self.rows.iter(),
            row_parser: &self.row_parser,
        }
    }
}

pub struct ParsedRowIterator<'a> {
    rows_iter: std::slice::Iter<'a, String>,
    row_parser: &'a RowParser,
}

impl Iterator for ParsedRowIterator<'_> {
    type Item = ParsedRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows_iter.next().map(|row| self.row_parser.parse(row))
    }
}
