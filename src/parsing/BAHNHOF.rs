use std::{collections::HashMap, error::Error, rc::Rc};

use crate::models::Stop;

use super::{ColumnDefinition, ExpectedType, FileParser, SingleConfigurationRowParser};

pub fn load_stops() -> Result<Vec<Rc<Stop>>, Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(13, -1, ExpectedType::String),
    ];
    let row_parser = SingleConfigurationRowParser::new(row_configuration);
    let file_parser = FileParser::new("data/BAHNHOF", Box::new(row_parser))?;

    Ok(file_parser
        .iter()
        .map(|(_, mut values)| {
            let id = i32::from(values.remove(0));
            let raw_name = String::from(values.remove(0));

            let parsed_name = super::parse_stop_name(raw_name);

            let name = parsed_name.get(&1).unwrap()[0].clone();
            let long_name = parsed_name.get(&2).map(|x| x[0].clone());
            let abbreviation = parsed_name.get(&3).map(|x| x[0].clone());
            let synonyms = parsed_name.get(&4).cloned();

            Rc::new(Stop::new(id, name, long_name, abbreviation, synonyms))
        })
        .collect())
}

pub fn create_stops_primary_index(stops: &Vec<Rc<Stop>>) -> HashMap<i32, Rc<Stop>> {
    stops.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.id, Rc::clone(item));
        acc
    })
}
