use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use chrono::NaiveDate;
use strum_macros::{self, Display, EnumString};

// ------------------------------------------------------------------------------------------------
// --- Attribute
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub struct Attribute {
    id: String,
    stop_scope: i16,
    main_sorting_priority: i16,
    secondary_sorting_priority: i16,
    description: RefCell<HashMap<String, String>>, // Key: deu, fra, ita or eng.
}

#[allow(unused)]
impl Attribute {
    pub fn new(
        id: String,
        stop_scope: i16,
        main_sorting_priority: i16,
        secondary_sorting_priority: i16,
    ) -> Self {
        Self {
            id,
            stop_scope,
            main_sorting_priority,
            secondary_sorting_priority,
            description: RefCell::new(HashMap::new()),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn stop_scope(&self) -> i16 {
        self.stop_scope
    }

    pub fn main_sorting_priority(&self) -> i16 {
        self.main_sorting_priority
    }

    pub fn secondary_sorting_priority(&self) -> i16 {
        self.secondary_sorting_priority
    }

    pub fn description(&self, language: Language) -> String {
        self.description
            .borrow()
            .get(&language.to_string())
            .cloned()
            .unwrap()
    }

    pub fn set_description(&self, language: Language, value: &str) {}
}

// ------------------------------------------------------------------------------------------------
// --- BitField
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub struct BitField {
    id: i32,
    // TODO : find a better name, perhaps?
    values: Vec<u8>,
}

#[allow(unused)]
impl BitField {
    pub fn new(id: i32, values: Vec<u8>) -> Self {
        Self { id, values }
    }

    pub fn id(&self) -> i32 {
        return self.id;
    }

    pub fn values(&self) -> &Vec<u8> {
        return &self.values;
    }
}

// ------------------------------------------------------------------------------------------------
// --- Coordinate
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CoordinateType {
    #[default]
    LV95,
    WGS84,
}

#[allow(unused)]
#[derive(Debug, Default)]
pub struct Coordinate {
    // TODO : should I add a getter for the field?
    coordinate_type: CoordinateType,
    x: f64,
    y: f64,
    z: i16,
}

#[allow(unused)]
impl Coordinate {
    pub fn new(coordinate_type: CoordinateType, x: f64, y: f64, z: i16) -> Self {
        Self {
            coordinate_type,
            x,
            y,
            z,
        }
    }

    pub fn easting(&self) -> f64 {
        assert!(self.coordinate_type == CoordinateType::LV95);
        self.x
    }

    pub fn northing(&self) -> f64 {
        assert!(self.coordinate_type == CoordinateType::LV95);
        self.y
    }

    pub fn latitude(&self) -> f64 {
        assert!(self.coordinate_type == CoordinateType::WGS84);
        self.x
    }

    pub fn longitude(&self) -> f64 {
        assert!(self.coordinate_type == CoordinateType::WGS84);
        self.y
    }

    pub fn altitude(&self) -> i16 {
        self.z
    }
}

// ------------------------------------------------------------------------------------------------
// --- Holiday
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub struct Holiday {
    date: NaiveDate,
    name: HashMap<String, String>, // Key: deu, fra, ita or eng.
}

#[allow(unused)]
impl Holiday {
    pub fn new(date: NaiveDate, name: HashMap<String, String>) -> Self {
        Self { date, name }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn name(&self, language: Language) -> String {
        self.name.get(&language.to_string()).cloned().unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// --- Language
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Clone, Copy, Debug, Default, Display, EnumString)]
pub enum Language {
    #[default]
    #[strum(serialize = "deu")]
    German,

    #[strum(serialize = "fra")]
    French,

    #[strum(serialize = "ita")]
    Italian,

    #[strum(serialize = "eng")]
    English,
}

// ------------------------------------------------------------------------------------------------
// --- JourneyPlatform
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub struct JourneyPlatform {
    journey_id: i32,
    platform_id: i64,
    unknown1: String, // "Verwaltung für Fahrt"
    hour: Option<i16>,
    bit_field_id: Option<i32>,
}

#[allow(unused)]
impl JourneyPlatform {
    pub fn new(
        journey_id: i32,
        platform_id: i64,
        unknown1: String,
        hour: Option<i16>,
        bit_field_id: Option<i32>,
    ) -> Self {
        Self {
            journey_id,
            platform_id,
            unknown1,
            hour,
            bit_field_id,
        }
    }

    pub fn journey_id(&self) -> i32 {
        self.journey_id
    }

    pub fn platform_id(&self) -> i64 {
        self.platform_id
    }

    pub fn unknown1(&self) -> &str {
        &self.unknown1
    }

    pub fn hour(&self) -> &Option<i16> {
        &self.hour
    }

    pub fn bit_field_id(&self) -> &Option<i32> {
        &self.bit_field_id
    }
}

// ------------------------------------------------------------------------------------------------
// --- Platform
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug, Default)]
pub struct Platform {
    id: i64, // Haltestellennummer << 32 + "Index der Gleistextinformation"
    code: String,
    sectors: Option<String>,
    sloid: RefCell<String>,
    lv95_coordinate: RefCell<Coordinate>,
    wgs84_coordinate: RefCell<Coordinate>,
}

#[allow(unused)]
impl Platform {
    pub fn new(id: i64, code: String, sectors: Option<String>) -> Self {
        Self {
            id,
            code,
            sectors,
            sloid: RefCell::new(String::default()),
            lv95_coordinate: RefCell::new(Coordinate::default()),
            wgs84_coordinate: RefCell::new(Coordinate::default()),
        }
    }

    pub fn create_id(stop_id: i32, stop_id_index: i32) -> i64 {
        ((stop_id as i64) << 32) + (stop_id_index as i64)
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn sectors(&self) -> &Option<String> {
        &self.sectors
    }

    pub fn sloid(&self) -> Ref<'_, String> {
        self.sloid.borrow()
    }

    pub fn set_sloid(&self, value: String) {
        *self.sloid.borrow_mut() = value;
    }

    pub fn lv95_coordinate(&self) -> Ref<'_, Coordinate> {
        self.lv95_coordinate.borrow()
    }

    pub fn set_lv95_coordinate(&self, value: Coordinate) {
        *self.lv95_coordinate.borrow_mut() = value;
    }

    pub fn wgs84_coordinate(&self) -> Ref<'_, Coordinate> {
        self.wgs84_coordinate.borrow()
    }

    pub fn set_wgs84_coordinate(&self, value: Coordinate) {
        *self.wgs84_coordinate.borrow_mut() = value;
    }
}

// ------------------------------------------------------------------------------------------------
// --- Stop
// ------------------------------------------------------------------------------------------------

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
    changing_priority: RefCell<i16>,
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
            changing_priority: RefCell::new(0),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
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

    pub fn set_lv95_coordinate(&self, value: Coordinate) {
        *self.lv95_coordinate.borrow_mut() = Some(value);
    }

    pub fn wgs84_coordinate(&self) -> Ref<'_, Option<Coordinate>> {
        self.wgs84_coordinate.borrow()
    }

    pub fn set_wgs84_coordinate(&self, value: Coordinate) {
        *self.wgs84_coordinate.borrow_mut() = Some(value);
    }

    pub fn changing_priority(&self) -> i16 {
        *self.changing_priority.borrow()
    }

    pub fn set_changing_priority(&self, value: i16) {
        *self.changing_priority.borrow_mut() = value;
    }
}

// ------------------------------------------------------------------------------------------------
// --- TimetableKeyData
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug)]
pub struct TimetableKeyData {
    start_date: NaiveDate, // The date is included.
    end_date: NaiveDate,   // The date is included.
    metadata: Vec<String>,
}

#[allow(unused)]
impl TimetableKeyData {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate, metadata: Vec<String>) -> Self {
        Self {
            start_date,
            end_date,
            metadata,
        }
    }

    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    pub fn metadata(&self) -> &Vec<String> {
        &self.metadata
    }
}
