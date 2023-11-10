use std::{collections::HashMap, error::Error};

use crate::{
    models::{Coordinate, Stop},
    parsing::{self, ColumnDefinition, ExpectedType, FileParser, SingleConfigurationRowParser},
};

pub struct Hrdf {
    pub stops: HashMap<i32, Stop>,
}

impl Hrdf {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            stops: Self::load_stops()?,
        })
    }

    // BAHNHOF
    fn load_stops() -> Result<HashMap<i32, Stop>, Box<dyn Error>> {
        let row_configuration = vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(13, -1, ExpectedType::String),
        ];
        let row_parser = SingleConfigurationRowParser::new(row_configuration);
        let file_parser = FileParser::new("data/BAHNHOF", Box::new(row_parser))?;

        let mut stops = file_parser
            .iter()
            .map(|mut values| {
                let id = i32::from(values.remove(0));
                let raw_name = String::from(values.remove(0));

                let parsed_name = parsing::parse_stop_name(raw_name);

                let name = parsed_name.get(&1).unwrap()[0].clone();
                let long_name = parsed_name.get(&2).map(|x| x[0].clone());
                let abbreviation = parsed_name.get(&3).map(|x| x[0].clone());
                let synonyms = parsed_name.get(&4).cloned();

                Stop::new(id, name, long_name, abbreviation, synonyms)
            })
            .fold(HashMap::new(), |mut acc, stop| {
                acc.insert(stop.id, stop);
                acc
            });

        Self::load_wgs_coordinates(&mut stops)?;
        Self::load_lv95_coordinates(&mut stops)?;

        Ok(stops)
    }

    // BFKOORD_WGS
    fn load_wgs_coordinates(stops: &mut HashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
        let row_configuration = vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 18, ExpectedType::Float),
            ColumnDefinition::new(20, 29, ExpectedType::Float),
            ColumnDefinition::new(31, 36, ExpectedType::Integer16),
        ];
        let row_parser = SingleConfigurationRowParser::new(row_configuration);
        let file_parser = FileParser::new("data/BFKOORD_WGS", Box::new(row_parser))?;

        for mut values in file_parser.iter() {
            let stop_id = i32::from(values.remove(0));

            if let Some(stop) = stops.get_mut(&stop_id) {
                let x = f64::from(values.remove(0));
                let y = f64::from(values.remove(0));
                let z = i16::from(values.remove(0));

                stop.wgs_coordinate = Some(Coordinate::new(x, y, z));
            }
        }

        Ok(())
    }

    // BFKOORD_LV95
    fn load_lv95_coordinates(stops: &mut HashMap<i32, Stop>) -> Result<(), Box<dyn Error>> {
        let row_configuration = vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 18, ExpectedType::Float),
            ColumnDefinition::new(20, 29, ExpectedType::Float),
            ColumnDefinition::new(31, 36, ExpectedType::Integer16),
        ];
        let row_parser = SingleConfigurationRowParser::new(row_configuration);
        let file_parser = FileParser::new("data/BFKOORD_LV95", Box::new(row_parser))?;

        for mut values in file_parser.iter() {
            let stop_id = i32::from(values.remove(0));

            if let Some(stop) = stops.get_mut(&stop_id) {
                let y = f64::from(values.remove(0));
                let x = f64::from(values.remove(0));
                let z = i16::from(values.remove(0));

                stop.lv95_coordinate = Some(Coordinate::new(x, y, z));
            }
        }

        Ok(())
    }
}
