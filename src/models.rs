use std::cell::{Ref, RefCell};

#[allow(unused)]
#[derive(Debug)]
pub struct Stop {
    id: i32,
    name: String,
    long_name: Option<String>,
    abbreviation: Option<String>,
    synonyms: Option<Vec<String>>,
    lv95_coordinate: RefCell<Option<Coordinate>>,
    wgs84_coordinate: RefCell<Option<Coordinate>>,
}

#[allow(unused)]
impl Stop {
    pub fn new(
        id: i32,
        name: String,
        long_name: Option<String>,
        abbreviation: Option<String>,
        synonyms: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            name,
            long_name,
            abbreviation,
            synonyms,
            lv95_coordinate: RefCell::new(None),
            wgs84_coordinate: RefCell::new(None),
        }
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn long_name(&self) -> &Option<String> {
        &self.long_name
    }

    pub fn abbreviation(&self) -> &Option<String> {
        &self.abbreviation
    }

    pub fn synonyms(&self) -> &Option<Vec<String>> {
        &self.synonyms
    }

    pub fn lv95_coordinate(&self) -> Ref<'_, Option<Coordinate>> {
        self.lv95_coordinate.borrow()
    }

    pub fn set_lv95_coordinate(&self, coordinate: Coordinate) {
        *self.lv95_coordinate.borrow_mut() = Some(coordinate);
    }

    pub fn wgs84_coordinate(&self) -> Ref<'_, Option<Coordinate>> {
        self.wgs84_coordinate.borrow()
    }

    pub fn set_wgs84_coordinate(&self, coordinate: Coordinate) {
        *self.wgs84_coordinate.borrow_mut() = Some(coordinate);
    }
}

#[derive(Debug, PartialEq)]
pub enum CoordinateType {
    LV95,
    WGS84,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Coordinate {
    // TODO : should I add a getter for the field?
    coordinate_type: CoordinateType,
    x: f64,
    y: f64,
    z: i16,
    // TODO : is this field useless?
    stop_id: i32,
}

#[allow(unused)]
impl Coordinate {
    pub fn new(coordinate_type: CoordinateType, x: f64, y: f64, z: i16, stop_id: i32) -> Self {
        Self {
            coordinate_type,
            x,
            y,
            z,
            stop_id,
        }
    }

    pub fn easting(&self) -> &f64 {
        assert!(self.coordinate_type == CoordinateType::LV95);
        &self.x
    }

    pub fn northing(&self) -> &f64 {
        assert!(self.coordinate_type == CoordinateType::LV95);
        &self.y
    }

    pub fn latitude(&self) -> &f64 {
        assert!(self.coordinate_type == CoordinateType::WGS84);
        &self.x
    }

    pub fn longitude(&self) -> &f64 {
        assert!(self.coordinate_type == CoordinateType::WGS84);
        &self.y
    }

    pub fn altitude(&self) -> &i16 {
        &self.z
    }
}

// TODO :

#[allow(unused)]
#[derive(Debug)]
pub struct JourneyPlatform {
    pub journey_id: i32,
    pub stop_id: i32,
    unknown1: String,
    platform_index: i32,
}

#[allow(unused)]
impl JourneyPlatform {
    pub fn new(journey_id: i32, stop_id: i32, unknown1: String, platform_index: i32) -> Self {
        Self {
            stop_id,
            journey_id,
            unknown1,
            platform_index,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Platform {
    pub stop_id: i32,
    pub platform_index: i32,
    data: String,
    sloid: RefCell<String>,
}

impl Platform {
    pub fn new(stop_id: i32, platform_index: i32, data: String) -> Self {
        Self {
            stop_id,
            platform_index,
            data,
            sloid: RefCell::new("".to_string()),
        }
    }
}
