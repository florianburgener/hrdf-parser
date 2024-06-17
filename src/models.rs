use std::{
    collections::{BTreeSet, HashMap},
    hash::{DefaultHasher, Hash, Hasher},
};

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use strum_macros::{self, Display, EnumString};

use crate::storage::DataStorage;

// ------------------------------------------------------------------------------------------------
// --- Model
// ------------------------------------------------------------------------------------------------

pub trait Model<M: Model<M>> {
    // Primary key type.
    type K: Eq + Hash + Serialize + for<'a> Deserialize<'a>;

    fn id(&self) -> M::K;

    fn vec_to_map(data: Vec<M>) -> HashMap<M::K, M> {
        data.into_iter().fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.id(), item);
            acc
        })
    }
}

macro_rules! impl_Model {
    ($m:ty) => {
        impl Model<$m> for $m {
            type K = i32;

            fn id(&self) -> Self::K {
                self.id
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// --- Attribute
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    id: i32,
    designation: String,
    stop_scope: i16,
    main_sorting_priority: i16,
    secondary_sorting_priority: i16,
    description: HashMap<Language, String>,
}

impl_Model!(Attribute);

impl Attribute {
    pub fn new(
        id: i32,
        designation: String,
        stop_scope: i16,
        main_sorting_priority: i16,
        secondary_sorting_priority: i16,
    ) -> Self {
        Self {
            id,
            designation,
            stop_scope,
            main_sorting_priority,
            secondary_sorting_priority,
            description: HashMap::new(),
        }
    }

    // Getters/Setters

    pub fn set_description(&mut self, language: Language, value: &str) {
        self.description.insert(language, value.to_string());
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- BitField
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct BitField {
    id: i32,
    bits: Vec<u8>,
}

impl_Model!(BitField);

impl BitField {
    pub fn new(id: i32, bits: Vec<u8>) -> Self {
        Self { id, bits }
    }

    // Getters/Setters

    pub fn bits(&self) -> &Vec<u8> {
        &self.bits
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- Color
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
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

    // Getters/Setters

    pub fn r(&self) -> i16 {
        self.r
    }

    pub fn g(&self) -> i16 {
        self.g
    }

    pub fn b(&self) -> i16 {
        self.b
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- CoordinateType
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum CoordinateType {
    #[default]
    LV95,
    WGS84,
}

// ------------------------------------------------------------------------------------------------
// --- Coordinate
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Coordinate {
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

    // Getters/Setters

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

    // TODO: most stops have no specified altitude.
    pub fn altitude(&self) -> i16 {
        self.z
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- Direction
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Direction {
    id: i32,
    name: String,
}

impl_Model!(Direction);

impl Direction {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    // Getters/Setters

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- DirectionType
// ------------------------------------------------------------------------------------------------

#[derive(
    Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, EnumString, Serialize, Deserialize,
)]
pub enum DirectionType {
    #[default]
    #[strum(serialize = "R")]
    Outbound,

    #[strum(serialize = "H")]
    Return,
}

// ------------------------------------------------------------------------------------------------
// --- Holiday
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Holiday {
    id: i32,
    date: NaiveDate,
    name: HashMap<Language, String>,
}

impl_Model!(Holiday);

impl Holiday {
    pub fn new(id: i32, date: NaiveDate, name: HashMap<Language, String>) -> Self {
        Self { id, date, name }
    }

    // Getters/Setters

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- InformationText
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct InformationText {
    id: i32,
    content: HashMap<Language, String>,
}

impl_Model!(InformationText);

impl InformationText {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            content: HashMap::new(),
        }
    }

    // Getters/Setters

    pub fn set_content(&mut self, language: Language, value: &str) {
        self.content.insert(language, value.to_string());
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- Journey
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Journey {
    id: i32,
    administration: String,
    metadata: HashMap<JourneyMetadataType, Vec<JourneyMetadataEntry>>,
    route: Vec<JourneyRouteEntry>,
}

impl_Model!(Journey);

impl Journey {
    pub fn new(id: i32, administration: String) -> Self {
        Self {
            id,
            administration,
            metadata: HashMap::new(),
            route: Vec::new(),
        }
    }

    // Getters/Setters

    fn metadata(&self) -> &HashMap<JourneyMetadataType, Vec<JourneyMetadataEntry>> {
        &self.metadata
    }

    pub fn route(&self) -> &Vec<JourneyRouteEntry> {
        &self.route
    }

    // Functions

    pub fn add_metadata_entry(&mut self, k: JourneyMetadataType, v: JourneyMetadataEntry) {
        self.metadata.entry(k).or_insert(Vec::new()).push(v);
    }

    pub fn add_route_entry(&mut self, entry: JourneyRouteEntry) {
        self.route.push(entry);
    }

    pub fn bit_field<'a>(&'a self, data_storage: &'a DataStorage) -> Option<&BitField> {
        let entry = &self.metadata().get(&JourneyMetadataType::BitField).unwrap()[0];

        entry
            .bit_field_id
            .map(|bit_field_id| data_storage.bit_fields().find(bit_field_id))
    }

    pub fn first_stop_id(&self) -> i32 {
        self.route.first().unwrap().stop_id()
    }

    pub fn last_stop_id(&self) -> i32 {
        self.route.last().unwrap().stop_id()
    }

    pub fn is_last_stop(&self, stop_id: i32, ignore_loop: bool) -> bool {
        if ignore_loop && self.first_stop_id() == self.last_stop_id() {
            false
        } else {
            stop_id == self.last_stop_id()
        }
    }

    pub fn count_stops(&self, departure_stop_id: i32, arrival_stop_id: i32) -> i32 {
        let mut route_iter = self.route().iter().peekable();

        while route_iter.peek().unwrap().stop_id() != departure_stop_id {
            route_iter.next();
        }

        let mut count = 0;

        loop {
            count += 1;
            let stop_id = route_iter.next().unwrap().stop_id();

            if stop_id == arrival_stop_id {
                break;
            }
        }

        count
    }

    pub fn hash_route(&self, departure_stop_id: i32) -> Option<u64> {
        let mut route_iter = self.route().iter().peekable();

        loop {
            if let Some(route_entry) = route_iter.peek() {
                if route_entry.stop_id() == departure_stop_id {
                    break;
                }
            } else {
                return None;
            }

            route_iter.next();
        }

        let mut hasher = DefaultHasher::new();
        route_iter
            .map(|route_entry| route_entry.stop_id())
            .collect::<BTreeSet<_>>()
            .hash(&mut hasher);
        Some(hasher.finish())
    }

    pub fn departure_time_of(&self, stop_id: i32) -> NaiveTime {
        self.route()
            .iter()
            .find(|route_entry| route_entry.stop_id() == stop_id)
            .unwrap()
            .departure_time()
            .unwrap()
    }

    pub fn arrival_time_of(&self, stop_id: i32) -> NaiveTime {
        self.route()
            .iter()
            .skip(1)
            .find(|route_entry| route_entry.stop_id() == stop_id)
            .unwrap()
            .arrival_time()
            .unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyMetadataType
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum JourneyMetadataType {
    #[default]
    Attribute,
    BitField,
    Direction,
    InformationText,
    Line,
    TransferTimeBoarding,
    TransferTimeDisembarking,
    TransportType,
}

// ------------------------------------------------------------------------------------------------
// --- JourneyMetadataEntry
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyMetadataEntry {
    from_stop_id: Option<i32>,
    until_stop_id: Option<i32>,
    resource_id: Option<i32>,
    bit_field_id: Option<i32>,
    departure_time: Option<NaiveTime>,
    arrival_time: Option<NaiveTime>,
    extra_field_1: Option<String>,
    extra_field_2: Option<i32>,
}

impl JourneyMetadataEntry {
    pub fn new(
        from_stop_id: Option<i32>,
        until_stop_id: Option<i32>,
        resource_id: Option<i32>,
        bit_field_id: Option<i32>,
        departure_time: Option<NaiveTime>,
        arrival_time: Option<NaiveTime>,
        extra_field_1: Option<String>,
        extra_field_2: Option<i32>,
    ) -> Self {
        Self {
            from_stop_id,
            until_stop_id,
            resource_id,
            bit_field_id,
            departure_time,
            arrival_time,
            extra_field_1,
            extra_field_2,
        }
    }

    // Getters/Setters

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- JourneyRouteEntry
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyRouteEntry {
    stop_id: i32,
    arrival_time: Option<NaiveTime>,
    departure_time: Option<NaiveTime>,
}

impl JourneyRouteEntry {
    pub fn new(
        stop_id: i32,
        arrival_time: Option<NaiveTime>,
        departure_time: Option<NaiveTime>,
    ) -> Self {
        Self {
            stop_id,
            arrival_time,
            departure_time,
        }
    }

    // Getters/Setters

    pub fn stop_id(&self) -> i32 {
        self.stop_id
    }

    pub fn arrival_time(&self) -> &Option<NaiveTime> {
        &self.arrival_time
    }

    pub fn departure_time(&self) -> &Option<NaiveTime> {
        &self.departure_time
    }

    // Functions

    pub fn stop<'a>(&'a self, data_storage: &'a DataStorage) -> &Stop {
        data_storage.stops().find(self.stop_id())
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyPlatform
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyPlatform {
    journey_id: i32,
    platform_id: i32,
    time: Option<NaiveTime>,
    bit_field_id: Option<i32>,
}

impl JourneyPlatform {
    pub fn new(
        journey_id: i32,
        platform_id: i32,
        time: Option<NaiveTime>,
        bit_field_id: Option<i32>,
    ) -> Self {
        Self {
            journey_id,
            platform_id,
            time,
            bit_field_id,
        }
    }

    // Getters/Setters

    // Functions
}

impl Model<JourneyPlatform> for JourneyPlatform {
    type K = (i32, i32);

    fn id(&self) -> Self::K {
        (self.journey_id, self.platform_id)
    }
}

// ------------------------------------------------------------------------------------------------
// --- Language
// ------------------------------------------------------------------------------------------------

#[derive(
    Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, EnumString, Serialize, Deserialize,
)]
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
// --- Line
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Line {
    id: i32,
    name: String,
    short_name: String,
    text_color: Color,
    background_color: Color,
}

impl_Model!(Line);

impl Line {
    pub fn new(id: i32, name: String) -> Self {
        Self {
            id,
            name,
            short_name: String::default(),
            text_color: Color::default(),
            background_color: Color::default(),
        }
    }

    // Getters/Setters

    pub fn set_short_name(&mut self, value: String) {
        self.short_name = value;
    }

    pub fn set_text_color(&mut self, value: Color) {
        self.text_color = value;
    }

    pub fn set_background_color(&mut self, value: Color) {
        self.background_color = value;
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- Platform
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    id: i32,
    name: String,
    sectors: Option<String>,
    stop_id: i32,
    sloid: String,
    lv95_coordinate: Coordinate,
    wgs84_coordinate: Coordinate,
}

impl_Model!(Platform);

impl Platform {
    pub fn new(id: i32, name: String, sectors: Option<String>, stop_id: i32) -> Self {
        Self {
            id,
            name,
            sectors,
            stop_id,
            sloid: String::default(),
            lv95_coordinate: Coordinate::default(),
            wgs84_coordinate: Coordinate::default(),
        }
    }

    // Getters/Setters

    pub fn set_sloid(&mut self, value: String) {
        self.sloid = value;
    }

    pub fn set_lv95_coordinate(&mut self, value: Coordinate) {
        self.lv95_coordinate = value;
    }

    pub fn set_wgs84_coordinate(&mut self, value: Coordinate) {
        self.wgs84_coordinate = value;
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- Stop
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    id: i32,
    name: String,
    long_name: Option<String>,
    abbreviation: Option<String>,
    synonyms: Option<Vec<String>>,
    lv95_coordinate: Option<Coordinate>,
    wgs84_coordinate: Option<Coordinate>,
    transfer_priority: i16,
    transfer_flag: i16,
    transfer_time_inter_city: i16,
    transfer_time_other: i16,
    connections: Vec<i32>, // Vec of Stop.id
    restrictions: i16,
    sloid: String,
    boarding_areas: Vec<String>,
}

impl_Model!(Stop);

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
            lv95_coordinate: None,
            wgs84_coordinate: None,
            transfer_priority: 8, // 8 is the default priority.
            transfer_flag: 0,
            transfer_time_inter_city: 0,
            transfer_time_other: 0,
            connections: Vec::default(),
            restrictions: 0,
            sloid: String::default(),
            boarding_areas: Vec::new(),
        }
    }

    // Getters/Setters

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_lv95_coordinate(&mut self, value: Coordinate) {
        self.lv95_coordinate = Some(value);
    }

    pub fn set_wgs84_coordinate(&mut self, value: Coordinate) {
        self.wgs84_coordinate = Some(value);
    }

    pub fn set_transfer_priority(&mut self, value: i16) {
        self.transfer_priority = value;
    }

    pub fn transfer_flag(&self) -> i16 {
        self.transfer_flag
    }

    pub fn set_transfer_flag(&mut self, value: i16) {
        self.transfer_flag = value;
    }

    pub fn set_transfer_time_inter_city(&mut self, value: i16) {
        self.transfer_time_inter_city = value;
    }

    pub fn set_transfer_time_other(&mut self, value: i16) {
        self.transfer_time_other = value;
    }

    pub fn set_connections(&mut self, value: Vec<i32>) {
        self.connections = value;
    }

    pub fn set_restrictions(&mut self, value: i16) {
        self.restrictions = value;
    }

    pub fn set_sloid(&mut self, value: String) {
        self.sloid = value;
    }

    // Functions

    pub fn add_boarding_area(&mut self, value: String) {
        self.boarding_areas.push(value);
    }

    pub fn can_be_used_as_exchange_point(&self) -> bool {
        self.transfer_flag() != 0
    }
}

// ------------------------------------------------------------------------------------------------
// --- StopConnection
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StopConnection {
    id: i32,
    stop_id_1: i32,
    stop_id_2: i32,
    duration: i16, // Transfer time from stop 1 to stop 2 is in minutes.
    attribute: i32,
}

impl_Model!(StopConnection);

impl StopConnection {
    pub fn new(id: i32, stop_id_1: i32, stop_id_2: i32, duration: i16) -> Self {
        Self {
            id,
            stop_id_1,
            stop_id_2,
            duration,
            attribute: 0,
        }
    }

    // Getters/Setters

    pub fn stop_id_1(&self) -> i32 {
        self.stop_id_1
    }

    pub fn stop_id_2(&self) -> i32 {
        self.stop_id_2
    }

    pub fn duration(&self) -> i16 {
        self.duration
    }

    pub fn set_attribute(&mut self, value: i32) {
        self.attribute = value;
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- ThroughService
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct ThroughService {
    id: i32,
    journey_1_id: i32,
    journey_1_stop_id: i32, // Last stop of journey 1.
    journey_2_id: i32,
    journey_2_stop_id: Option<i32>, // First stop of journey 2.
    bit_field_id: i32,
}

impl_Model!(ThroughService);

impl ThroughService {
    pub fn new(
        id: i32,
        journey_1_id: i32,
        journey_1_stop_id: i32,
        journey_2_id: i32,
        journey_2_stop_id: Option<i32>,
        bit_field_id: i32,
    ) -> Self {
        Self {
            id,
            journey_1_id,
            journey_1_stop_id,
            journey_2_id,
            journey_2_stop_id,
            bit_field_id,
        }
    }

    // Getters/Setters

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- TimetableMetadataEntry
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableMetadataEntry {
    id: i32,
    key: String,
    value: String,
}

impl_Model!(TimetableMetadataEntry);

impl TimetableMetadataEntry {
    pub fn new(id: i32, key: String, value: String) -> Self {
        Self { id, key, value }
    }

    // Getters/Setters

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    #[allow(non_snake_case)]
    pub fn value_as_NaiveDate(&self) -> NaiveDate {
        NaiveDate::parse_from_str(self.value(), "%Y-%m-%d").unwrap()
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- TransferTimeAdministration
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTimeAdministration {
    id: i32,
    stop_id: Option<i32>, // A None value means that the transfer time applies to all stops if there is no specific entry for the stop and the 2 administrations.
    administration_1: String,
    administration_2: String,
    duration: i16, // Transfer time from administration 1 to administration 2 is in minutes.
}

impl_Model!(TransferTimeAdministration);

impl TransferTimeAdministration {
    pub fn new(
        id: i32,
        stop_id: Option<i32>,
        administration_1: String,
        administration_2: String,
        duration: i16,
    ) -> Self {
        Self {
            id,
            stop_id,
            administration_1,
            administration_2,
            duration,
        }
    }

    // Getters/Setters

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- TransferTimeJourney
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTimeJourney {
    id: i32,
    stop_id: i32,
    journey_id_1: i32,
    journey_id_2: i32,
    duration: i16, // Transfer time from journey 1 to journey 2 is in minutes.
    is_guaranteed: bool,
    bit_field_id: Option<i32>,
}

impl_Model!(TransferTimeJourney);

impl TransferTimeJourney {
    pub fn new(
        id: i32,
        stop_id: i32,
        journey_id_1: i32,
        journey_id_2: i32,
        duration: i16,
        is_guaranteed: bool,
        bit_field_id: Option<i32>,
    ) -> Self {
        Self {
            id,
            stop_id,
            journey_id_1,
            journey_id_2,
            duration,
            is_guaranteed,
            bit_field_id,
        }
    }

    // Getters/Setters

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- TransferTimeLine
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferTimeLine {
    id: i32,
    stop_id: i32,
    administration_1: String,
    transport_type_id_1: i32,
    line_id_1: Option<String>, // If the value is None, then the transfer time applies to all lines in administration_1.
    direction_1: Option<DirectionType>, // If the value is None, then the match time applies in both directions.
    administration_2: String,
    transport_type_id_2: i32,
    line_id_2: Option<String>, // If the value is None, then the transfer time applies to all lines in administration_2.
    direction_2: Option<DirectionType>, // If the value is None, then the match time applies in both directions.
    duration: i16,                      // Transfer time from line 1 to line 2 is in minutes.
    is_guaranteed: bool,
}

impl_Model!(TransferTimeLine);

impl TransferTimeLine {
    pub fn new(
        id: i32,
        stop_id: i32,
        administration_1: String,
        transport_type_id_1: i32,
        line_id_1: Option<String>,
        direction_1: Option<DirectionType>,
        administration_2: String,
        transport_type_id_2: i32,
        line_id_2: Option<String>,
        direction_2: Option<DirectionType>,
        duration: i16,
        is_guaranteed: bool,
    ) -> Self {
        Self {
            id,
            stop_id,
            administration_1,
            transport_type_id_1,
            line_id_1,
            direction_1,
            administration_2,
            transport_type_id_2,
            line_id_2,
            direction_2,
            duration,
            is_guaranteed,
        }
    }

    // Getters/Setters

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- TransportCompany
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportCompany {
    id: i32,
    short_name: HashMap<Language, String>,
    long_name: HashMap<Language, String>,
    full_name: HashMap<Language, String>,
    administrations: Vec<String>,
}

impl_Model!(TransportCompany);

impl TransportCompany {
    pub fn new(id: i32, administrations: Vec<String>) -> Self {
        Self {
            id,
            short_name: HashMap::new(),
            long_name: HashMap::new(),
            full_name: HashMap::new(),
            administrations,
        }
    }

    // Getters/Setters

    pub fn set_short_name(&mut self, language: Language, value: &str) {
        self.short_name.insert(language, value.to_string());
    }

    pub fn set_long_name(&mut self, language: Language, value: &str) {
        self.long_name.insert(language, value.to_string());
    }

    pub fn set_full_name(&mut self, language: Language, value: &str) {
        self.full_name.insert(language, value.to_string());
    }

    // Functions
}

// ------------------------------------------------------------------------------------------------
// --- TransportType
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransportType {
    id: i32,
    designation: String,
    product_class_id: i16,
    tarrif_group: String,
    output_control: i16,
    short_name: String,
    surchage: i16,
    flag: String,
    product_class_name: HashMap<Language, String>,
    category_name: HashMap<Language, String>,
}

impl_Model!(TransportType);

impl TransportType {
    pub fn new(
        id: i32,
        designation: String,
        product_class_id: i16,
        tarrif_group: String,
        output_control: i16,
        short_name: String,
        surchage: i16,
        flag: String,
    ) -> Self {
        Self {
            id,
            designation,
            product_class_id,
            tarrif_group,
            output_control,
            short_name,
            surchage,
            flag,
            product_class_name: HashMap::new(),
            category_name: HashMap::new(),
        }
    }

    // Getters/Setters

    pub fn product_class_id(&self) -> i16 {
        self.product_class_id
    }

    pub fn set_product_class_name(&mut self, language: Language, value: &str) {
        self.product_class_name.insert(language, value.to_string());
    }

    pub fn set_category_name(&mut self, language: Language, value: &str) {
        self.category_name.insert(language, value.to_string());
    }

    // Functions
}
