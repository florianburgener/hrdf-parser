use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    rc::Rc,
};

use chrono::{NaiveDate, NaiveDateTime};
use strum_macros::{self, Display, EnumString};

// ------------------------------------------------------------------------------------------------
// --- Attribute
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Attribute {
    id: String,
    stop_scope: i16,
    main_sorting_priority: i16,
    secondary_sorting_priority: i16,
    description: RefCell<HashMap<String, String>>, // Key: deu, fra, ita or eng.
}

pub type AttributeCollection = Vec<Rc<Attribute>>;
pub type AttributePrimaryIndex = HashMap<String, Rc<Attribute>>;

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

    pub fn set_description(&self, language: Language, value: &str) {
        self.description
            .borrow_mut()
            .insert(language.to_string(), value.to_string());
    }
}

// ------------------------------------------------------------------------------------------------
// --- BitField
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct BitField {
    id: i32,
    // TODO : find a better name, perhaps?
    values: Vec<u8>,
}

pub type BitFieldCollection = Vec<Rc<BitField>>;
pub type BitFieldPrimaryIndex = HashMap<i32, Rc<BitField>>;

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
// --- Color
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct Color {
    r: i16,
    g: i16,
    b: i16,
}

#[allow(unused)]
impl Color {
    pub fn new(r: i16, g: i16, b: i16) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> i16 {
        self.r
    }

    pub fn g(&self) -> i16 {
        self.g
    }

    pub fn b(&self) -> i16 {
        self.b
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
// --- Direction
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Direction {
    id: String,
    name: String,
}

pub type DirectionCollection = Vec<Rc<Direction>>;
pub type DirectionPrimaryIndex = HashMap<String, Rc<Direction>>;

#[allow(unused)]
impl Direction {
    pub fn new(id: String, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

// ------------------------------------------------------------------------------------------------
// --- Holiday
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Holiday {
    date: NaiveDate,
    name: HashMap<String, String>, // Key: deu, fra, ita or eng.
}

pub type HolidayCollection = Vec<Rc<Holiday>>;

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
// --- InformationText
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct InformationText {
    id: i32,
    content: RefCell<HashMap<String, String>>, // Key: deu, fra, ita or eng.
}

pub type InformationTextCollection = Vec<Rc<InformationText>>;
pub type InformationTextPrimaryIndex = HashMap<i32, Rc<InformationText>>;

#[allow(unused)]
impl InformationText {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            content: RefCell::new(HashMap::new()),
        }
    }

    pub fn id(&self) -> i32 {
        return self.id;
    }

    pub fn content(&self, language: Language) -> String {
        self.content
            .borrow()
            .get(&language.to_string())
            .cloned()
            .unwrap()
    }

    pub fn set_content(&self, language: Language, value: &str) {
        self.content
            .borrow_mut()
            .insert(language.to_string(), value.to_string());
    }
}

// ------------------------------------------------------------------------------------------------
// --- Language
// ------------------------------------------------------------------------------------------------

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

#[derive(Debug)]
pub struct JourneyPlatform {
    journey_id: i32,
    platform_id: i64,
    unknown1: String, // "Verwaltung für Fahrt"
    hour: Option<i16>,
    bit_field_id: Option<i32>,
}

pub type JourneyPlatformCollection = Vec<Rc<JourneyPlatform>>;
pub type JourneyPlatformPrimaryIndex = HashMap<(i32, i64), Rc<JourneyPlatform>>;

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
// --- Line
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Line {
    id: i32,
    name: String,
    short_name: RefCell<String>,
    text_color: RefCell<Color>,
    background_color: RefCell<Color>,
}

pub type LineCollection = Vec<Rc<Line>>;
pub type LinePrimaryIndex = HashMap<i32, Rc<Line>>;

#[allow(unused)]
impl Line {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            short_name: RefCell::new(String::new()),
            text_color: RefCell::new(Color::default()),
            background_color: RefCell::new(Color::default()),
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn short_name(&self) -> Ref<'_, String> {
        self.short_name.borrow()
    }

    pub fn set_short_name(&self, value: String) {
        *self.short_name.borrow_mut() = value;
    }

    pub fn text_color(&self) -> Ref<'_, Color> {
        self.text_color.borrow()
    }

    pub fn set_text_color(&self, value: Color) {
        *self.text_color.borrow_mut() = value;
    }

    pub fn background_color(&self) -> Ref<'_, Color> {
        self.background_color.borrow()
    }

    pub fn set_background_color(&self, value: Color) {
        *self.background_color.borrow_mut() = value;
    }
}

// ------------------------------------------------------------------------------------------------
// --- Platform
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Platform {
    id: i64, // Haltestellennummer << 32 + "Index der Gleistextinformation"
    code: String,
    sectors: Option<String>,
    sloid: RefCell<String>,
    lv95_coordinate: RefCell<Coordinate>,
    wgs84_coordinate: RefCell<Coordinate>,
}

pub type PlatformCollection = Vec<Rc<Platform>>;
pub type PlatformPrimaryIndex = HashMap<i64, Rc<Platform>>;

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
    changing_flag: RefCell<Option<i16>>,
    changing_time_inter_city: RefCell<i16>,
    changing_time_other: RefCell<i16>,
    connections: RefCell<Vec<i32>>,
    restrictions: RefCell<i16>,
    sloid: RefCell<String>,
    boarding_areas: RefCell<Vec<String>>,
}

pub type StopCollection = Vec<Rc<Stop>>;
pub type StopPrimaryIndex = HashMap<i32, Rc<Stop>>;

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
            changing_priority: RefCell::new(8), // 8 is the default priority.
            changing_flag: RefCell::new(None),
            changing_time_inter_city: RefCell::new(0),
            changing_time_other: RefCell::new(0),
            connections: RefCell::new(Vec::new()),
            restrictions: RefCell::new(0),
            sloid: RefCell::new(String::new()),
            boarding_areas: RefCell::new(Vec::new()),
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

    pub fn changing_flag(&self) -> Option<i16> {
        *self.changing_flag.borrow()
    }

    pub fn set_changing_flag(&self, value: i16) {
        *self.changing_flag.borrow_mut() = Some(value);
    }

    pub fn changing_time_inter_city(&self) -> i16 {
        *self.changing_time_inter_city.borrow()
    }

    pub fn set_changing_time_inter_city(&self, value: i16) {
        *self.changing_time_inter_city.borrow_mut() = value;
    }

    pub fn changing_time_other(&self) -> i16 {
        *self.changing_time_other.borrow()
    }

    pub fn set_changing_time_other(&self, value: i16) {
        *self.changing_time_other.borrow_mut() = value;
    }

    pub fn connections(&self) -> Ref<'_, Vec<i32>> {
        self.connections.borrow()
    }

    pub fn set_connections(&self, value: Vec<i32>) {
        *self.connections.borrow_mut() = value;
    }

    pub fn restrictions(&self) -> Ref<'_, i16> {
        self.restrictions.borrow()
    }

    pub fn set_restrictions(&self, value: i16) {
        *self.restrictions.borrow_mut() = value;
    }

    pub fn sloid(&self) -> Ref<'_, String> {
        self.sloid.borrow()
    }

    pub fn set_sloid(&self, value: String) {
        *self.sloid.borrow_mut() = value;
    }

    pub fn boarding_areas(&self) -> Ref<'_, Vec<String>> {
        self.boarding_areas.borrow()
    }

    pub fn add_boarding_area(&self, value: String) {
        self.boarding_areas.borrow_mut().push(value);
    }
}

// ------------------------------------------------------------------------------------------------
// --- StopConnection
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct StopConnection {
    stop_id_1: i32,
    stop_id_2: i32,
    duration: i16, // Transfer time from stop 1 to stop 2 is in minutes.
    attributes: RefCell<Vec<String>>,
}

pub type StopConnectionCollection = Vec<Rc<StopConnection>>;
// TODO : primary index

#[allow(unused)]
impl StopConnection {
    pub fn new(stop_id_1: i32, stop_id_2: i32, duration: i16) -> Self {
        Self {
            stop_id_1,
            stop_id_2,
            duration,
            attributes: RefCell::new(Vec::new()),
        }
    }

    pub fn stop_id_1(&self) -> i32 {
        self.stop_id_1
    }

    pub fn stop_id_2(&self) -> i32 {
        self.stop_id_2
    }

    pub fn duration(&self) -> i16 {
        self.duration
    }

    pub fn attributes(&self) -> Ref<'_, Vec<String>> {
        self.attributes.borrow()
    }

    pub fn add_attribute(&self, value: String) {
        self.attributes.borrow_mut().push(value);
    }
}

// ------------------------------------------------------------------------------------------------
// --- TimeDifference
// ------------------------------------------------------------------------------------------------

#[allow(unused)] // TODO
#[derive(Debug)]
pub struct TimeDifference {
    stop_id: i32,
    time_zone: i32,
    time_zone_summer_1: Option<i32>,
    start_date_1: Option<NaiveDateTime>,
    end_date_1: Option<NaiveDateTime>,
    time_zone_summer_2: Option<i32>,
    start_date_2: Option<NaiveDateTime>,
    end_date_2: Option<NaiveDateTime>,
}

pub type TimeDifferenceCollection = Vec<Rc<TimeDifference>>;

#[allow(unused)]
impl TimeDifference {
    pub fn new(
        stop_id: i32,
        time_zone: i32,
        time_zone_summer_1: Option<i32>,
        start_date_1: Option<NaiveDateTime>,
        end_date_1: Option<NaiveDateTime>,
        time_zone_summer_2: Option<i32>,
        start_date_2: Option<NaiveDateTime>,
        end_date_2: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            stop_id,
            time_zone,
            time_zone_summer_1,
            start_date_1,
            end_date_1,
            time_zone_summer_2,
            start_date_2,
            end_date_2,
        }
    }

    // TODO
}

// ------------------------------------------------------------------------------------------------
// --- ThroughService
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct ThroughService {
    journey_1_id: i32,
    journey_1_unknown: String, // "Verwaltung für Fahrt 1"
    journey_1_stop_id: i32,    // Last stop of journey 1.
    journey_2_id: i32,
    journey_2_unknown: String,      // "Verwaltung für Fahrt 2"
    journey_2_stop_id: Option<i32>, // First stop of journey 2.
    bit_field_id: i32,
    // TODO : "Attribut zur Markierung der Durchbindung (optional)"
}

pub type ThroughServiceCollection = Vec<Rc<ThroughService>>;
// TODO : primary index

#[allow(unused)]
impl ThroughService {
    pub fn new(
        journey_1_id: i32,
        journey_1_unknown: String,
        journey_1_stop_id: i32,
        journey_2_id: i32,
        journey_2_unknown: String,
        journey_2_stop_id: Option<i32>,
        bit_field_id: i32,
    ) -> Self {
        Self {
            journey_1_id,
            journey_1_unknown,
            journey_1_stop_id,
            journey_2_id,
            journey_2_unknown,
            journey_2_stop_id,
            bit_field_id,
        }
    }

    pub fn journey_1_id(&self) -> i32 {
        self.journey_1_id
    }

    pub fn journey_1_unknown(&self) -> &str {
        &self.journey_1_unknown
    }

    pub fn journey_1_stop_id(&self) -> i32 {
        self.journey_1_stop_id
    }

    pub fn journey_2_id(&self) -> i32 {
        self.journey_2_id
    }

    pub fn journey_2_unknown(&self) -> &str {
        &self.journey_2_unknown
    }

    pub fn journey_2_stop_id(&self) -> &Option<i32> {
        &self.journey_2_stop_id
    }

    pub fn bit_field_id(&self) -> i32 {
        self.bit_field_id
    }
}

// ------------------------------------------------------------------------------------------------
// --- TimetableKeyData
// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------
// --- TransportCompany
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TransportCompany {
    id: i32,
    short_name: RefCell<HashMap<String, String>>, // Key: deu, fra, ita or eng.
    long_name: RefCell<HashMap<String, String>>,  // Key: deu, fra, ita or eng.
    full_name: RefCell<HashMap<String, String>>,  // Key: deu, fra, ita or eng.
    administrations: Vec<String>,
}

pub type TransportCompanyCollection = Vec<Rc<TransportCompany>>;
pub type TransportCompanyPrimaryIndex = HashMap<i32, Rc<TransportCompany>>;

#[allow(unused)]
impl TransportCompany {
    pub fn new(id: i32, administrations: Vec<String>) -> Self {
        Self {
            id,
            short_name: RefCell::new(HashMap::new()),
            long_name: RefCell::new(HashMap::new()),
            full_name: RefCell::new(HashMap::new()),
            administrations,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn short_name(&self, language: Language) -> String {
        self.short_name
            .borrow()
            .get(&language.to_string())
            .cloned()
            .unwrap()
    }

    pub fn set_short_name(&self, language: Language, value: &str) {
        self.short_name
            .borrow_mut()
            .insert(language.to_string(), value.to_string());
    }

    pub fn long_name(&self, language: Language) -> String {
        self.long_name
            .borrow()
            .get(&language.to_string())
            .cloned()
            .unwrap()
    }

    pub fn set_long_name(&self, language: Language, value: &str) {
        self.long_name
            .borrow_mut()
            .insert(language.to_string(), value.to_string());
    }

    pub fn full_name(&self, language: Language) -> String {
        self.full_name
            .borrow()
            .get(&language.to_string())
            .cloned()
            .unwrap()
    }

    pub fn set_full_name(&self, language: Language, value: &str) {
        self.full_name
            .borrow_mut()
            .insert(language.to_string(), value.to_string());
    }

    pub fn administrations(&self) -> &Vec<String> {
        &self.administrations
    }
}

// ------------------------------------------------------------------------------------------------
// --- TransportType
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TransportType {
    id: String,
    product_class_id: i16,
    tarrif_group: String,
    output_control: i16,
    short_name: String,
    surchage: i16,
    flag: String,
    product_class_name: RefCell<HashMap<String, String>>,
    long_name: RefCell<HashMap<String, String>>, // TODO : option10, option11, option12, ...
}

pub type TransportTypeCollection = Vec<Rc<TransportType>>;
pub type TransportTypePrimaryIndex = HashMap<String, Rc<TransportType>>;

#[allow(unused)]
impl TransportType {
    pub fn new(
        id: String,
        product_class_id: i16,
        tarrif_group: String,
        output_control: i16,
        short_name: String,
        surchage: i16,
        flag: String,
    ) -> Self {
        Self {
            id,
            product_class_id,
            tarrif_group,
            output_control,
            short_name,
            surchage,
            flag,
            product_class_name: RefCell::new(HashMap::new()),
            long_name: RefCell::new(HashMap::new()),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn product_class_id(&self) -> i16 {
        self.product_class_id
    }

    pub fn tarrif_group(&self) -> &str {
        &self.tarrif_group
    }

    pub fn output_control(&self) -> i16 {
        self.output_control
    }

    pub fn short_name(&self) -> &str {
        &self.short_name
    }

    pub fn surchage(&self) -> i16 {
        self.surchage
    }

    pub fn flag(&self) -> &str {
        &self.flag
    }

    pub fn product_class_name(&self, language: Language) -> String {
        self.product_class_name
            .borrow()
            .get(&language.to_string())
            .cloned()
            .unwrap()
    }

    pub fn set_product_class_name(&self, language: Language, value: &str) {
        self.product_class_name
            .borrow_mut()
            .insert(language.to_string(), value.to_string());
    }

    pub fn long_name(&self, language: Language) -> String {
        self.long_name
            .borrow()
            .get(&language.to_string())
            .cloned()
            .unwrap()
    }

    pub fn set_long_name(&self, language: Language, value: &str) {
        self.long_name
            .borrow_mut()
            .insert(language.to_string(), value.to_string());
    }
}
