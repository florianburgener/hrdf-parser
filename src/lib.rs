use std::{cmp, error::Error, fs, io};

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
        Self {
            start,
            stop,
            expected_type,
        }
    }
}

struct FileParser {
    rows: Vec<String>,
    row_configuration: Vec<ColumnDefinition>,
}

impl FileParser {
    fn new(file_path: &str, row_configuration: Vec<ColumnDefinition>) -> io::Result<Self> {
        let contents = Self::read_file(file_path)?;
        let rows = contents.lines().map(String::from).collect();

        Ok(Self {
            rows,
            row_configuration,
        })
    }

    fn read_file(file_path: &str) -> io::Result<String> {
        fs::read_to_string(file_path)
    }

    fn iter(&self) -> FileParserIterator {
        FileParserIterator {
            parser: self,
            index: 0,
        }
    }

    fn parse_row(&self, raw_row: &str) -> Vec<ParsedValue> {
        let mut values = vec![];

        for column_definition in &self.row_configuration {
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
}

struct FileParserIterator<'a> {
    parser: &'a FileParser,
    index: usize,
}

impl<'a> Iterator for FileParserIterator<'a> {
    type Item = Vec<ParsedValue>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.parser.rows.len() {
            let raw_row = &self.parser.rows[self.index];
            let item = Some(self.parser.parse_row(raw_row));
            self.index += 1;
            item
        } else {
            None
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(13, 62, ExpectedType::String),
    ];

    let parser = FileParser::new("A.txt", row_configuration)?;

    for (index, values) in parser.iter().enumerate() {
        println!("Row {} :", index + 1);

        for value in &values {
            println!("    {:?}", value);
        }
    }

    Ok(())
}
