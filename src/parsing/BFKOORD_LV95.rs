use std::{collections::HashMap, error::Error, rc::Rc};

use crate::models::Lv95Coordinate;

use super::{ColumnDefinition, ExpectedType, FileParser, SingleConfigurationRowParser};

pub fn load_lv95_stop_coordinates() -> Result<Vec<Rc<Lv95Coordinate>>, Box<dyn Error>> {
    let row_configuration = vec![
        ColumnDefinition::new(1, 7, ExpectedType::Integer32),
        ColumnDefinition::new(9, 18, ExpectedType::Float),
        ColumnDefinition::new(20, 29, ExpectedType::Float),
        ColumnDefinition::new(31, 36, ExpectedType::Integer16),
    ];
    let row_parser = SingleConfigurationRowParser::new(row_configuration);
    let file_parser = FileParser::new("data/BFKOORD_LV95", Box::new(row_parser))?;

    Ok(file_parser
        .iter()
        .map(|(_, mut values)| {
            let stop_id = i32::from(values.remove(0));
            let easting = f64::from(values.remove(0));
            let northing = f64::from(values.remove(0));
            let altitude = i16::from(values.remove(0));

            Rc::new(Lv95Coordinate::new(easting, northing, altitude, stop_id))
        })
        .collect())
}

pub fn create_lv95_stop_coordinates_index_1(
    coordinates: &Vec<Rc<Lv95Coordinate>>,
) -> HashMap<i32, Rc<Lv95Coordinate>> {
    coordinates.iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item.stop_id, Rc::clone(item));
        acc
    })
}
