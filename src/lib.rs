mod models;
mod parser;

use parser::ColumnDefinition;
use parser::DefaultRowParser;
use parser::ExpectedType;
use parser::FileParser;
use std::error::Error;

use crate::models::Stop;

pub fn run() -> Result<(), Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(13, -1, ExpectedType::String),
    ];
    let row_parser = DefaultRowParser::new(row_configuration);
    let parser = FileParser::new("A.txt", Box::new(row_parser))?;

    let mut stops: Vec<Stop> = vec![];

    for (index, mut values) in parser.iter().enumerate() {
        println!("Row {} : {:?}", index + 1, values);

        let id = i32::from(values.remove(0));
        let raw_name = String::from(values.remove(0));

        let parsed_name = parser::parse_stop_name(raw_name);
        let name = parsed_name.get(&1).unwrap()[0].clone();

        stops.push(Stop::new(id, name));
    }

    println!("Stops:\n{:?}", stops);
    Ok(())
}
