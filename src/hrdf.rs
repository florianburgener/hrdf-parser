use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{Lv95Coordinate, Stop, WgsCoordinate},
    parsing::{
        self, ColumnDefinition, ExpectedType, FileParser, MultipleConfigurationRowParser, RowType,
        SingleConfigurationRowParser,
    },
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    // Tables
    stops: Vec<Rc<Stop>>,
    lv95_stop_coordinates: Vec<Rc<Lv95Coordinate>>,
    wgs_stop_coordinates: Vec<Rc<WgsCoordinate>>,
    // Indexes
    stops_primary_index: HashMap<i32, Rc<Stop>>,
    lv95_stop_coordinates_index_1: HashMap<i32, Rc<Lv95Coordinate>>,
    wgs_stop_coordinates_index_1: HashMap<i32, Rc<WgsCoordinate>>,
}

impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let stops = Self::load_stops()?;
        let lv95_stop_coordinates = Self::load_lv95_stop_coordinates()?;
        let wgs_stop_coordinates = Self::load_wgs_stop_coordinates()?;
        Self::load_journey_stop_and_platforms()?;

        let stops_primary_index = Self::create_stops_primary_index(&stops);
        let lv95_stop_coordinates_index_1 =
            Self::create_lv95_stop_coordinates_index_1(&lv95_stop_coordinates);
        let wgs_stop_coordinates_index_1 =
            Self::create_wgs_stop_coordinates_index_1(&wgs_stop_coordinates);

        let instance = Rc::new(Self {
            // Tables
            stops,
            lv95_stop_coordinates,
            wgs_stop_coordinates,
            // Indexes
            stops_primary_index,
            lv95_stop_coordinates_index_1,
            wgs_stop_coordinates_index_1,
        });

        Self::set_parent_references(&instance);
        Ok(instance)
    }

    // BAHNHOF
    fn load_stops() -> Result<Vec<Rc<Stop>>, Box<dyn Error>> {
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

                let parsed_name = parsing::parse_stop_name(raw_name);

                let name = parsed_name.get(&1).unwrap()[0].clone();
                let long_name = parsed_name.get(&2).map(|x| x[0].clone());
                let abbreviation = parsed_name.get(&3).map(|x| x[0].clone());
                let synonyms = parsed_name.get(&4).cloned();

                Rc::new(Stop::new(id, name, long_name, abbreviation, synonyms))
            })
            .collect())
    }

    // BFKOORD_LV95 (BF = BAHNHOF)
    fn load_lv95_stop_coordinates() -> Result<Vec<Rc<Lv95Coordinate>>, Box<dyn Error>> {
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

    // BFKOORD_WGS (BF = BAHNHOF)
    fn load_wgs_stop_coordinates() -> Result<Vec<Rc<WgsCoordinate>>, Box<dyn Error>> {
        let row_configuration = vec![
            ColumnDefinition::new(1, 7, ExpectedType::Integer32),
            ColumnDefinition::new(9, 18, ExpectedType::Float),
            ColumnDefinition::new(20, 29, ExpectedType::Float),
            ColumnDefinition::new(31, 36, ExpectedType::Integer16),
        ];
        let row_parser = SingleConfigurationRowParser::new(row_configuration);
        let file_parser = FileParser::new("data/BFKOORD_WGS", Box::new(row_parser))?;

        Ok(file_parser
            .iter()
            .map(|(_, mut values)| {
                let stop_id = i32::from(values.remove(0));
                let longitude = f64::from(values.remove(0));
                let latitude = f64::from(values.remove(0));
                let altitude = i16::from(values.remove(0));

                Rc::new(WgsCoordinate::new(latitude, longitude, altitude, stop_id))
            })
            .collect())
    }

    // GLEIS
    fn load_journey_stop_and_platforms() -> Result<(), Box<dyn Error>> {
        const GLEIS_ROW_A: i32 = 1;
        const GLEIS_ROW_B: i32 = 2;

        #[rustfmt::skip]
        let row_types = vec![
            RowType::new(GLEIS_ROW_A, 8, 9, "#", false, vec![
                ColumnDefinition::new(1, 7, ExpectedType::Integer32),
                ColumnDefinition::new(9, 14, ExpectedType::Integer32),
                ColumnDefinition::new(16, 21, ExpectedType::String),
            ]),
            RowType::new(GLEIS_ROW_B, 8, 9, "#", true, vec![
                ColumnDefinition::new(1, 7, ExpectedType::Integer32),
                ColumnDefinition::new(10, 16, ExpectedType::Integer32),
                ColumnDefinition::new(18, -1, ExpectedType::String),
            ]),
        ];
        let row_parser = MultipleConfigurationRowParser::new(row_types);
        let file_parser = FileParser::new("data/GLEIS", Box::new(row_parser))?;

        for (_id, _values) in file_parser.iter() {
            // if id == GLEIS_ROW_B {
            //     println!("{:?}", values);
            // }
        }

        Ok(())
    }

    fn create_stops_primary_index(stops: &Vec<Rc<Stop>>) -> HashMap<i32, Rc<Stop>> {
        stops.iter().fold(HashMap::new(), |mut acc, stop| {
            acc.insert(stop.id, Rc::clone(stop));
            acc
        })
    }

    fn create_lv95_stop_coordinates_index_1(
        coordinates: &Vec<Rc<Lv95Coordinate>>,
    ) -> HashMap<i32, Rc<Lv95Coordinate>> {
        coordinates
            .iter()
            .fold(HashMap::new(), |mut acc, coordinate| {
                acc.insert(coordinate.stop_id, Rc::clone(coordinate));
                acc
            })
    }

    fn create_wgs_stop_coordinates_index_1(
        coordinates: &Vec<Rc<WgsCoordinate>>,
    ) -> HashMap<i32, Rc<WgsCoordinate>> {
        coordinates
            .iter()
            .fold(HashMap::new(), |mut acc, coordinate| {
                acc.insert(coordinate.stop_id, Rc::clone(coordinate));
                acc
            })
    }

    fn set_parent_references(instance: &Rc<Hrdf>) {
        for stop in &instance.stops {
            stop.set_parent_reference(&instance);
        }
    }

    pub fn stops(&self) -> &Vec<Rc<Stop>> {
        &self.stops
    }

    pub fn stops_primary_index(&self) -> &HashMap<i32, Rc<Stop>> {
        &self.stops_primary_index
    }

    pub fn lv95_stop_coordinates_index_1(&self) -> &HashMap<i32, Rc<Lv95Coordinate>> {
        &self.lv95_stop_coordinates_index_1
    }

    pub fn wgs_stop_coordinates_index_1(&self) -> &HashMap<i32, Rc<WgsCoordinate>> {
        &self.wgs_stop_coordinates_index_1
    }
}
