use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    hash::Hash,
    rc::Rc,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use strum_macros::{self, Display, EnumString};

// ------------------------------------------------------------------------------------------------
// --- Model
// ------------------------------------------------------------------------------------------------

pub trait Model<M: Model<M>> {
    // Primary key type.
    type K: Eq + Hash + Serialize + for<'a> Deserialize<'a>;

    fn id(&self) -> M::K;

    fn create_primary_index(rows: &ResourceCollection<M>) -> ResourceIndex<M, M::K> {
        rows.iter().fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.id(), Rc::clone(item));
            acc
        })
    }
}

/// M = Model type.
pub type ResourceCollection<M> = Vec<Rc<M>>;

/// M = Model type.
/// K = Key type.
pub type ResourceIndex<M, K = i32> = HashMap<K, Rc<M>>;

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
    description: RefCell<HashMap<Language, String>>,
}

impl Model<Attribute> for Attribute {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
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
            description: RefCell::new(HashMap::new()),
        }
    }

    pub fn designation(&self) -> &str {
        &self.designation
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
        self.description.borrow().get(&language).cloned().unwrap()
    }

    pub fn set_description(&self, language: Language, value: &str) {
        self.description
            .borrow_mut()
            .insert(language, value.to_string());
    }
}

// ------------------------------------------------------------------------------------------------
// --- BitField
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct BitField {
    id: i32,
    bits: Vec<u8>,
}

impl Model<BitField> for BitField {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl BitField {
    pub fn new(id: i32, bits: Vec<u8>) -> Self {
        Self { id, bits }
    }

    pub fn bits(&self) -> &Vec<u8> {
        return &self.bits;
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Direction {
    id: i32,
    name: String,
}

impl Model<Direction> for Direction {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl Direction {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
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

impl Model<Holiday> for Holiday {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl Holiday {
    pub fn new(id: i32, date: NaiveDate, name: HashMap<Language, String>) -> Self {
        Self { id, date, name }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn name(&self, language: Language) -> String {
        self.name.get(&language).cloned().unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// --- InformationText
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct InformationText {
    id: i32,
    content: RefCell<HashMap<Language, String>>,
}

impl Model<InformationText> for InformationText {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl InformationText {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            content: RefCell::new(HashMap::new()),
        }
    }

    pub fn content(&self, language: Language) -> String {
        self.content.borrow().get(&language).cloned().unwrap()
    }

    pub fn set_content(&self, language: Language, value: &str) {
        self.content
            .borrow_mut()
            .insert(language, value.to_string());
    }
}

// ------------------------------------------------------------------------------------------------
// --- Journey
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Journey {
    id: i32,
    administration: String,
    metadata: RefCell<HashMap<JourneyMetadataType, RefCell<Vec<JourneyMetadataEntry>>>>,
    route: RefCell<Vec<JourneyRouteEntry>>,
}

impl Model<Journey> for Journey {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl Journey {
    pub fn new(id: i32, administration: String) -> Self {
        Self {
            id,
            administration,
            metadata: RefCell::new(HashMap::new()),
            route: RefCell::new(Vec::new()),
        }
    }

    pub fn administration(&self) -> &str {
        &self.administration
    }

    pub fn add_metadata_entry(&self, k: JourneyMetadataType, v: JourneyMetadataEntry) {
        self.metadata
            .borrow_mut()
            .entry(k)
            .or_insert(RefCell::new(Vec::new()))
            .borrow_mut()
            .push(v);
    }

    pub fn add_route_entry(&self, entry: JourneyRouteEntry) {
        self.route.borrow_mut().push(entry);
    }

    // TODO: getters/setters.
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
    departure_time: Option<Time>,
    arrival_time: Option<Time>,
    extra_field_1: Option<String>,
    extra_field_2: Option<i32>,
}

#[allow(unused)]
impl JourneyMetadataEntry {
    pub fn new(
        from_stop_id: Option<i32>,
        until_stop_id: Option<i32>,
        resource_id: Option<i32>,
        bit_field_id: Option<i32>,
        departure_time: Option<Time>,
        arrival_time: Option<Time>,
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

    pub fn departure_time(&self) -> &Option<Time> {
        &self.departure_time
    }

    pub fn arrival_time(&self) -> &Option<Time> {
        &self.arrival_time
    }

    pub fn extra_field_1(&self) -> &Option<String> {
        &self.extra_field_1
    }

    pub fn extra_field_2(&self) -> &Option<i32> {
        &self.extra_field_2
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyRouteEntry
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyRouteEntry {
    stop_id: i32,
    arrival_time: Option<Time>,
    departure_time: Option<Time>,
    // TODO: add time type.
}

#[allow(unused)]
impl JourneyRouteEntry {
    pub fn new(stop_id: i32, arrival_time: Option<Time>, departure_time: Option<Time>) -> Self {
        Self {
            stop_id,
            arrival_time,
            departure_time,
        }
    }

    pub fn arrival_time(&self) -> &Option<Time> {
        &self.arrival_time
    }

    pub fn departure_time(&self) -> &Option<Time> {
        &self.departure_time
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyPlatform
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyPlatform {
    journey_id: i32,
    platform_id: i32,
    time: Option<Time>,
    bit_field_id: Option<i32>,
}

impl Model<JourneyPlatform> for JourneyPlatform {
    type K = (i32, i32);

    fn id(&self) -> Self::K {
        (self.journey_id, self.platform_id)
    }
}

#[allow(unused)]
impl JourneyPlatform {
    pub fn new(
        journey_id: i32,
        platform_id: i32,
        time: Option<Time>,
        bit_field_id: Option<i32>,
    ) -> Self {
        Self {
            journey_id,
            platform_id,
            time,
            bit_field_id,
        }
    }

    pub fn time(&self) -> &Option<Time> {
        &self.time
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
    short_name: RefCell<String>,
    text_color: RefCell<Color>,
    background_color: RefCell<Color>,
}

impl Model<Line> for Line {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    id: i32,
    name: String,
    sectors: Option<String>,
    stop_id: i32,
    sloid: RefCell<String>,
    lv95_coordinate: RefCell<Coordinate>,
    wgs84_coordinate: RefCell<Coordinate>,
}

impl Model<Platform> for Platform {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl Platform {
    pub fn new(id: i32, name: String, sectors: Option<String>, stop_id: i32) -> Self {
        Self {
            id,
            name,
            sectors,
            stop_id,
            sloid: RefCell::new(String::default()),
            lv95_coordinate: RefCell::new(Coordinate::default()),
            wgs84_coordinate: RefCell::new(Coordinate::default()),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    id: i32,
    name: String,
    long_name: Option<String>,
    abbreviation: Option<String>,
    synonyms: Option<Vec<String>>,
    lv95_coordinate: RefCell<Option<Coordinate>>,
    wgs84_coordinate: RefCell<Option<Coordinate>>,
    transfer_priority: RefCell<i16>,
    transfer_flag: RefCell<Option<i16>>,
    transfer_time_inter_city: RefCell<i16>,
    transfer_time_other: RefCell<i16>,
    connections: RefCell<Vec<i32>>, // Vec of Stop.id
    restrictions: RefCell<i16>,
    sloid: RefCell<String>,
    boarding_areas: RefCell<Vec<String>>,
}

impl Model<Stop> for Stop {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
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
            transfer_priority: RefCell::new(8), // 8 is the default priority.
            transfer_flag: RefCell::new(None),
            transfer_time_inter_city: RefCell::new(0),
            transfer_time_other: RefCell::new(0),
            connections: RefCell::new(Vec::new()),
            restrictions: RefCell::new(0),
            sloid: RefCell::new(String::new()),
            boarding_areas: RefCell::new(Vec::new()),
        }
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

    pub fn transfer_priority(&self) -> i16 {
        *self.transfer_priority.borrow()
    }

    pub fn set_transfer_priority(&self, value: i16) {
        *self.transfer_priority.borrow_mut() = value;
    }

    pub fn transfer_flag(&self) -> Option<i16> {
        *self.transfer_flag.borrow()
    }

    pub fn set_transfer_flag(&self, value: i16) {
        *self.transfer_flag.borrow_mut() = Some(value);
    }

    pub fn transfer_time_inter_city(&self) -> i16 {
        *self.transfer_time_inter_city.borrow()
    }

    pub fn set_transfer_time_inter_city(&self, value: i16) {
        *self.transfer_time_inter_city.borrow_mut() = value;
    }

    pub fn transfer_time_other(&self) -> i16 {
        *self.transfer_time_other.borrow()
    }

    pub fn set_transfer_time_other(&self, value: i16) {
        *self.transfer_time_other.borrow_mut() = value;
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StopConnection {
    id: i32,
    stop_id_1: i32,
    stop_id_2: i32,
    duration: i16, // Transfer time from stop 1 to stop 2 is in minutes.
    attributes: RefCell<Vec<i32>>,
}

impl Model<StopConnection> for StopConnection {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl StopConnection {
    pub fn new(id: i32, stop_id_1: i32, stop_id_2: i32, duration: i16) -> Self {
        Self {
            id,
            stop_id_1,
            stop_id_2,
            duration,
            attributes: RefCell::new(Vec::new()),
        }
    }

    pub fn duration(&self) -> i16 {
        self.duration
    }

    pub fn attributes(&self) -> Ref<'_, Vec<i32>> {
        self.attributes.borrow()
    }

    pub fn add_attribute(&self, value: i32) {
        self.attributes.borrow_mut().push(value);
    }
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

impl Model<ThroughService> for ThroughService {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
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
}

// ------------------------------------------------------------------------------------------------
// --- Time
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Time {
    hour: i8,
    minute: i8,
}

impl Time {
    pub fn new(hour: i8, minute: i8) -> Self {
        Self { hour, minute }
    }
}

impl From<i32> for Time {
    fn from(value: i32) -> Self {
        let value = value.abs();
        Time::new(
            i8::try_from(value / 100).unwrap(),
            i8::try_from(value % 100).unwrap(),
        )
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minute.partial_cmp(&other.minute)
    }
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

impl Model<TimetableMetadataEntry> for TimetableMetadataEntry {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
impl TimetableMetadataEntry {
    pub fn new(id: i32, key: String, value: String) -> Self {
        Self { id, key, value }
    }

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

impl Model<TransferTimeAdministration> for TransferTimeAdministration {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
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

    pub fn administration_1(&self) -> &str {
        &self.administration_1
    }

    pub fn administration_2(&self) -> &str {
        &self.administration_2
    }

    pub fn duration(&self) -> i16 {
        self.duration
    }
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

impl Model<TransferTimeJourney> for TransferTimeJourney {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
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

    pub fn duration(&self) -> i16 {
        self.duration
    }

    pub fn is_guaranteed(&self) -> bool {
        self.is_guaranteed
    }
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

impl Model<TransferTimeLine> for TransferTimeLine {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
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

    pub fn administration_1(&self) -> &str {
        &self.administration_1
    }

    pub fn direction_1(&self) -> &Option<DirectionType> {
        &self.direction_1
    }

    pub fn administration_2(&self) -> &str {
        &self.administration_2
    }

    pub fn direction_2(&self) -> &Option<DirectionType> {
        &self.direction_2
    }

    pub fn duration(&self) -> i16 {
        self.duration
    }

    pub fn is_guaranteed(&self) -> bool {
        self.is_guaranteed
    }
}

// ------------------------------------------------------------------------------------------------
// --- TransportCompany
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TransportCompany {
    id: i32,
    short_name: RefCell<HashMap<Language, String>>,
    long_name: RefCell<HashMap<Language, String>>,
    full_name: RefCell<HashMap<Language, String>>,
    administrations: Vec<String>,
}

impl Model<TransportCompany> for TransportCompany {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

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

    pub fn short_name(&self, language: Language) -> String {
        self.short_name.borrow().get(&language).cloned().unwrap()
    }

    pub fn set_short_name(&self, language: Language, value: &str) {
        self.short_name
            .borrow_mut()
            .insert(language, value.to_string());
    }

    pub fn long_name(&self, language: Language) -> String {
        self.long_name.borrow().get(&language).cloned().unwrap()
    }

    pub fn set_long_name(&self, language: Language, value: &str) {
        self.long_name
            .borrow_mut()
            .insert(language, value.to_string());
    }

    pub fn full_name(&self, language: Language) -> String {
        self.full_name.borrow().get(&language).cloned().unwrap()
    }

    pub fn set_full_name(&self, language: Language, value: &str) {
        self.full_name
            .borrow_mut()
            .insert(language, value.to_string());
    }

    pub fn administrations(&self) -> &Vec<String> {
        &self.administrations
    }
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
    product_class_name: RefCell<HashMap<Language, String>>,
    category_name: RefCell<HashMap<Language, String>>,
}

impl Model<TransportType> for TransportType {
    type K = i32;

    fn id(&self) -> Self::K {
        self.id
    }
}

#[allow(unused)]
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
            product_class_name: RefCell::new(HashMap::new()),
            category_name: RefCell::new(HashMap::new()),
        }
    }

    pub fn designation(&self) -> &str {
        &self.designation
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
            .get(&language)
            .cloned()
            .unwrap()
    }

    pub fn set_product_class_name(&self, language: Language, value: &str) {
        self.product_class_name
            .borrow_mut()
            .insert(language, value.to_string());
    }

    pub fn category_name(&self, language: Language) -> String {
        self.category_name.borrow().get(&language).cloned().unwrap()
    }

    pub fn set_category_name(&self, language: Language, value: &str) {
        self.category_name
            .borrow_mut()
            .insert(language, value.to_string());
    }
}
