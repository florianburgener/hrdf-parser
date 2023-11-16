use std::{collections::HashMap, error::Error, rc::Rc};

use crate::{
    models::{JourneyStop, Lv95Coordinate, Platform, Stop, WgsCoordinate},
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
    journey_stop_platforms: Vec<Rc<JourneyStop>>,
    platforms: Vec<Rc<Platform>>,

    // Indexes
    stops_primary_index: HashMap<i32, Rc<Stop>>, // Key = Haltestellennummer
    lv95_stop_coordinates_index_1: HashMap<i32, Rc<Lv95Coordinate>>, // Key = Haltestellennummer
    wgs_stop_coordinates_index_1: HashMap<i32, Rc<WgsCoordinate>>, // Key = Haltestellennummer
    journey_stop_platforms_index_1: HashMap<(i32, i32), Vec<Rc<JourneyStop>>>, // Key = (Haltestellennummer, Fahrtnummer)
    platforms_primary_index: HashMap<(i32, i32), Rc<Platform>>, // Key = (Haltestellennummer, Index der Gleistextinformation)
}

impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let stops = Self::load_stops()?;
        let lv95_stop_coordinates = Self::load_lv95_stop_coordinates()?;
        let wgs_stop_coordinates = Self::load_wgs_stop_coordinates()?;
        let (journey_stop_platforms, platforms) =
            Self::load_journey_stop_platforms_and_platforms()?;

        let stops_primary_index = Self::create_stops_primary_index(&stops);
        let lv95_stop_coordinates_index_1 =
            Self::create_lv95_stop_coordinates_index_1(&lv95_stop_coordinates);
        let wgs_stop_coordinates_index_1 =
            Self::create_wgs_stop_coordinates_index_1(&wgs_stop_coordinates);
        let journey_stop_platforms_index_1 =
            Self::create_journey_stop_platforms_index_1(&journey_stop_platforms);
        let platforms_primary_index = Self::create_platforms_primary_index(&platforms);

        let instance = Rc::new(Self {
            // Tables
            stops,
            lv95_stop_coordinates,
            wgs_stop_coordinates,
            journey_stop_platforms,
            platforms,
            // Indexes
            stops_primary_index,
            lv95_stop_coordinates_index_1,
            wgs_stop_coordinates_index_1,
            journey_stop_platforms_index_1,
            platforms_primary_index,
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
    fn load_journey_stop_platforms_and_platforms(
    ) -> Result<(Vec<Rc<JourneyStop>>, Vec<Rc<Platform>>), Box<dyn Error>> {
        const GLEIS_ROW_A: i32 = 1;
        const GLEIS_ROW_B: i32 = 2;

        #[rustfmt::skip]
        let row_types = vec![
            RowType::new(GLEIS_ROW_A, 8, 9, "#", false, vec![
                ColumnDefinition::new(1, 7, ExpectedType::Integer32),
                ColumnDefinition::new(9, 14, ExpectedType::Integer32),
                ColumnDefinition::new(16, 21, ExpectedType::String),
                // 23-30 with #
                ColumnDefinition::new(24, 30, ExpectedType::Integer32),
            ]),
            RowType::new(GLEIS_ROW_B, 8, 9, "#", true, vec![
                ColumnDefinition::new(1, 7, ExpectedType::Integer32),
                // 9-16 with #
                ColumnDefinition::new(10, 16, ExpectedType::Integer32),
                ColumnDefinition::new(18, -1, ExpectedType::String),
            ]),
        ];
        let row_parser = MultipleConfigurationRowParser::new(row_types);
        let file_parser = FileParser::new("data/GLEIS", Box::new(row_parser))?;

        let mut journey_stop_platforms = vec![];
        let mut platforms = vec![];

        for (id, mut values) in file_parser.iter() {
            match id {
                GLEIS_ROW_A => {
                    let stop_id = i32::from(values.remove(0));
                    let journey_id = i32::from(values.remove(0));

                    journey_stop_platforms.push(Rc::new(JourneyStop::new(
                        stop_id,
                        journey_id,
                        String::from(values.remove(0)),
                        i32::from(values.remove(0)),
                    )))
                }
                GLEIS_ROW_B => platforms.push(Rc::new(Platform::new(
                    i32::from(values.remove(0)),
                    i32::from(values.remove(0)),
                    String::from(values.remove(0)),
                ))),
                _ => unreachable!(),
            }
        }

        Ok((journey_stop_platforms, platforms))
    }

    fn create_stops_primary_index(stops: &Vec<Rc<Stop>>) -> HashMap<i32, Rc<Stop>> {
        stops.iter().fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.id, Rc::clone(item));
            acc
        })
    }

    fn create_lv95_stop_coordinates_index_1(
        coordinates: &Vec<Rc<Lv95Coordinate>>,
    ) -> HashMap<i32, Rc<Lv95Coordinate>> {
        coordinates.iter().fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.stop_id, Rc::clone(item));
            acc
        })
    }

    fn create_wgs_stop_coordinates_index_1(
        coordinates: &Vec<Rc<WgsCoordinate>>,
    ) -> HashMap<i32, Rc<WgsCoordinate>> {
        coordinates.iter().fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.stop_id, Rc::clone(item));
            acc
        })
    }

    fn create_journey_stop_platforms_index_1(
        journey_stop_platforms: &Vec<Rc<JourneyStop>>,
    ) -> HashMap<(i32, i32), Vec<Rc<JourneyStop>>> {
        journey_stop_platforms
            .iter()
            .fold(HashMap::new(), |mut acc, item| {
                acc.entry((item.journey_id, item.stop_id))
                    .or_insert(Vec::new())
                    .push(Rc::clone(item));
                acc
            })
    }

    fn create_platforms_primary_index(
        platforms: &Vec<Rc<Platform>>,
    ) -> HashMap<(i32, i32), Rc<Platform>> {
        platforms.iter().fold(HashMap::new(), |mut acc, item| {
            acc.insert((item.stop_id, item.platform_index), Rc::clone(item));
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
