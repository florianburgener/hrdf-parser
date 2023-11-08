use std::error::Error;

use crate::{
    models::Stop,
    parser::{self, ColumnDefinition, DefaultRowParser, ExpectedType, FileParser},
};

pub struct Hrdf {
    pub stops: Vec<Stop>,
}

impl Hrdf {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            stops: Self::load_stops()?,
        })
    }

    fn load_stops() -> Result<Vec<Stop>, Box<dyn Error>> {
        let row_configuration = vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ];
        let row_parser = DefaultRowParser::new(row_configuration);
        let parser = FileParser::new("A.txt", Box::new(row_parser))?;

        let mut stops: Vec<Stop> = vec![];

        for mut values in parser.iter() {
            let id = i32::from(values.remove(0));
            let raw_name = String::from(values.remove(0));

            let parsed_name = parser::parse_stop_name(raw_name);

            let name = parsed_name.get(&1).unwrap()[0].clone();
            let long_name = parsed_name.get(&2).map(|x| x[0].clone());
            let abbreviation = parsed_name.get(&3).map(|x| x[0].clone());
            let synonyms = parsed_name.get(&4).cloned();

            stops.push(Stop::new(id, name, long_name, abbreviation, synonyms));
        }

        Ok(stops)
    }
}
