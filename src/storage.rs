use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        Attribute, BitField, Direction, HasDataStorage, Holiday, InformationText, Journey,
        JourneyPlatform, Line, Model, Platform, Stop, StopConnection, ThroughService,
        TimetableMetadataEntry, TransferTimeAdministration, TransferTimeJourney, TransferTimeLine,
        TransportCompany, TransportType,
    },
    parsing,
    utils::count_days_between_two_dates,
};

// ------------------------------------------------------------------------------------------------
// --- DataStorage
// ------------------------------------------------------------------------------------------------

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
pub struct DataStorage {
    // Time-relevant data.
    bit_fields: SimpleResourceStorage<BitField>,
    holidays: SimpleResourceStorage<Holiday>,
    timetable_metadata: TimetableMetadataStorage,

    // Master data.
    attributes: SimpleResourceStorage<Attribute>,
    information_texts: SimpleResourceStorage<InformationText>,
    directions: SimpleResourceStorage<Direction>,
    lines: SimpleResourceStorage<Line>,
    transport_companies: SimpleResourceStorage<TransportCompany>,
    transport_types: SimpleResourceStorage<TransportType>,

    // Stop data.
    stops: SimpleResourceStorage<Stop>,
    stop_connections: SimpleResourceStorage<StopConnection>,

    // Timetable data.
    journeys: JourneyStorage,
    journey_platform: SimpleResourceStorage<JourneyPlatform>,
    platforms: SimpleResourceStorage<Platform>,
    through_service: SimpleResourceStorage<ThroughService>,

    // Transfer times.
    transfer_times_administration: SimpleResourceStorage<TransferTimeAdministration>,
    transfer_times_journey: SimpleResourceStorage<TransferTimeJourney>,
    transfer_times_line: SimpleResourceStorage<TransferTimeLine>,
}

#[allow(unused)]
impl DataStorage {
    pub fn new() -> Result<Rc<RefCell<Self>>, Box<dyn Error>> {
        // Time-relevant data.
        let bit_fields = parsing::load_bit_fields()?;
        let holidays = parsing::load_holidays()?;
        let timetable_metadata = parsing::load_timetable_metadata()?;

        // Master data.
        let (attributes, attributes_pk_type_converter) = parsing::load_attributes()?;
        let (directions, directions_pk_type_converter) = parsing::load_directions()?;
        let information_texts = parsing::load_information_texts()?;
        let lines = parsing::load_lines()?;
        let transport_companies = parsing::load_transport_companies()?;
        let (transport_types, transport_types_pk_type_converter) = parsing::load_transport_types()?;

        // Stop data.
        let stops = parsing::load_stops()?;
        let stop_connections = parsing::load_stop_connections(&attributes_pk_type_converter)?;

        // Timetable data.
        let (journeys, journeys_pk_type_converter) = parsing::load_journeys(
            &transport_types_pk_type_converter,
            &attributes_pk_type_converter,
            &directions_pk_type_converter,
        )?;
        let (journey_platform, platforms) = parsing::load_platforms(&journeys_pk_type_converter)?;
        let through_service = parsing::load_through_service(&journeys_pk_type_converter)?;

        // Transfer times.
        let transfer_times_administration = parsing::load_transfer_times_administration()?;
        let transfer_times_journey =
            parsing::load_transfer_times_journey(&journeys_pk_type_converter)?;
        let transfer_times_line =
            parsing::load_transfer_times_line(&transport_types_pk_type_converter)?;

        let data_storage = Rc::new(RefCell::new(Self {
            // Time-relevant data.
            bit_fields,
            holidays,
            timetable_metadata,
            // Master data.
            attributes,
            information_texts,
            directions,
            lines,
            transport_companies,
            transport_types,
            // Stop data.
            stops,
            stop_connections,
            // Timetable data.
            journeys,
            journey_platform,
            platforms,
            through_service,
            // Transfer times.
            transfer_times_administration,
            transfer_times_journey,
            transfer_times_line,
        }));

        data_storage.borrow_mut().set_references(&data_storage);
        Self::build_indexes(&data_storage);
        Ok(data_storage)
    }

    pub fn set_references(&mut self, data_storage: &Rc<RefCell<DataStorage>>) {
        self.journeys
            .entries_mut()
            .into_iter()
            .for_each(|item| item.set_data_storage_reference(data_storage));
    }

    pub fn remove_references(&mut self) {
        self.journeys
            .entries_mut()
            .into_iter()
            .for_each(|item| item.remove_data_storage_reference());
    }

    fn build_indexes(data_storage: &Rc<RefCell<DataStorage>>) {
        let indexes = data_storage.borrow().create_indexes();
        data_storage.borrow_mut().set_indexes(indexes);
    }

    fn create_indexes(&self) -> (JourneyIndexes,) {
        let journey_indexes = self.journeys().create_indexes(self);
        (journey_indexes,)
    }

    fn set_indexes(&mut self, indexes: (JourneyIndexes,)) {
        let (journey_indexes,) = indexes;
        self.journeys.set_indexes(journey_indexes);
    }

    pub fn bit_fields(&self) -> &SimpleResourceStorage<BitField> {
        &self.bit_fields
    }

    pub fn timetable_metadata(&self) -> &TimetableMetadataStorage {
        &self.timetable_metadata
    }

    pub fn journeys(&self) -> &JourneyStorage {
        &self.journeys
    }

    pub fn platforms(&self) -> &SimpleResourceStorage<Platform> {
        &self.platforms
    }

    pub fn stops(&self) -> &SimpleResourceStorage<Stop> {
        &self.stops
    }
}

// ------------------------------------------------------------------------------------------------
// --- SimpleResourceStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleResourceStorage<M: Model<M>> {
    data: HashMap<M::K, M>,
}

#[allow(unused)]
impl<M: Model<M>> SimpleResourceStorage<M> {
    pub fn new(data: HashMap<M::K, M>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &HashMap<M::K, M> {
        &self.data
    }

    pub fn find(&self, k: M::K) -> &M {
        &self.data().get(&k).unwrap()
    }
}

// ------------------------------------------------------------------------------------------------
// --- TimetableMetadataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableMetadataStorage {
    data: HashMap<i32, TimetableMetadataEntry>,
    timetable_metadata_entry_by_key: HashMap<String, i32>,
}

#[allow(unused)]
impl TimetableMetadataStorage {
    pub fn new(data: HashMap<i32, TimetableMetadataEntry>) -> Self {
        let timetable_metadata_entry_by_key = Self::create_timetable_metadata_entry_by_key(&data);

        Self {
            data,
            timetable_metadata_entry_by_key,
        }
    }

    fn data(&self) -> &HashMap<i32, TimetableMetadataEntry> {
        &self.data
    }

    fn timetable_metadata_entry_by_key(&self) -> &HashMap<String, i32> {
        &self.timetable_metadata_entry_by_key
    }

    fn create_timetable_metadata_entry_by_key(
        data: &HashMap<i32, TimetableMetadataEntry>,
    ) -> HashMap<String, i32> {
        data.values().fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.key().to_owned(), item.id());
            acc
        })
    }

    pub fn find(&self, id: i32) -> &TimetableMetadataEntry {
        self.data().get(&id).unwrap()
    }

    pub fn find_by_key(&self, key: &str) -> &TimetableMetadataEntry {
        self.find(*self.timetable_metadata_entry_by_key().get(key).unwrap())
    }

    pub fn start_date(&self) -> NaiveDate {
        self.find_by_key("start_date").value_as_NaiveDate()
    }

    pub fn end_date(&self) -> NaiveDate {
        self.find_by_key("end_date").value_as_NaiveDate()
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyStorage {
    data: HashMap<i32, Journey>,
    journeys_by_day: HashMap<NaiveDate, HashSet<i32>>,
    journeys_by_stop_id: HashMap<i32, HashSet<i32>>,
}

#[allow(unused)]
impl JourneyStorage {
    pub fn new(data: HashMap<i32, Journey>) -> Self {
        Self {
            data,
            journeys_by_day: HashMap::new(),
            journeys_by_stop_id: HashMap::new(),
        }
    }

    // Getters/Setters

    fn data(&self) -> &HashMap<i32, Journey> {
        &self.data
    }

    fn journeys_by_day(&self) -> &JourneyIndex1 {
        &self.journeys_by_day
    }

    fn set_journeys_by_day(&mut self, value: JourneyIndex1) {
        self.journeys_by_day = value;
    }

    fn journeys_by_stop_id(&self) -> &JourneyIndex2 {
        &self.journeys_by_stop_id
    }

    fn set_journeys_by_stop_id(&mut self, value: JourneyIndex2) {
        self.journeys_by_stop_id = value;
    }

    // Functions

    pub fn entries(&self) -> Vec<&Journey> {
        self.data().values().map(|j| j).collect()
    }

    pub fn entries_mut(&mut self) -> Vec<&mut Journey> {
        self.data.values_mut().map(|j| j).collect()
    }

    pub fn find(&self, id: i32) -> &Journey {
        self.data().get(&id).unwrap()
    }

    pub fn find_by_day(&self, day: NaiveDate) -> &HashSet<i32> {
        self.journeys_by_day().get(&day).unwrap()
    }

    pub fn find_by_stop_id(&self, stop_id: i32) -> &HashSet<i32> {
        self.journeys_by_stop_id().get(&stop_id).unwrap()
    }
}

type JourneyIndex1 = HashMap<NaiveDate, HashSet<i32>>;
type JourneyIndex2 = HashMap<i32, HashSet<i32>>;
type JourneyIndexes = (JourneyIndex1, JourneyIndex2);

// Manages the creation of indexes.
impl JourneyStorage {
    pub fn create_indexes(&self, data_storage: &DataStorage) -> JourneyIndexes {
        (
            self.create_journeys_by_day(data_storage),
            self.create_journeys_by_stop_id(data_storage),
        )
    }

    pub fn set_indexes(&mut self, indexes: JourneyIndexes) {
        let (journeys_by_day, journeys_by_stop_id) = indexes;

        self.set_journeys_by_day(journeys_by_day);
        self.set_journeys_by_stop_id(journeys_by_stop_id);
    }

    fn create_journeys_by_day(&self, data_storage: &DataStorage) -> JourneyIndex1 {
        let timetable_metadata = data_storage.timetable_metadata();
        let start_date = timetable_metadata.start_date();
        let num_days = count_days_between_two_dates(start_date, timetable_metadata.end_date());

        let dates: Vec<NaiveDate> = (0..num_days)
            .into_iter()
            .map(|i| {
                start_date
                    .checked_add_days(Days::new(i.try_into().unwrap()))
                    .unwrap()
            })
            .collect();

        self.entries().iter().fold(HashMap::new(), |mut acc, item| {
            let bit_field = item.bit_field();
            let indexes: Vec<usize> = if let Some(bit_field) = bit_field {
                bit_field
                    .bits()
                    .iter()
                    .enumerate()
                    .filter(|(i, &x)| *i < num_days && x == 1)
                    .map(|(i, _)| i)
                    .collect()
            } else {
                (0..num_days).collect()
            };

            indexes.into_iter().for_each(|i| {
                acc.entry(dates[i])
                    .or_insert(HashSet::new())
                    .insert(item.id());
            });

            acc
        })
    }

    fn create_journeys_by_stop_id(&self, _: &DataStorage) -> JourneyIndex2 {
        self.entries()
            .iter()
            .fold(HashMap::new(), |mut acc, journey| {
                journey.route().iter().for_each(|route_entry| {
                    acc.entry(route_entry.stop_id())
                        .or_insert(HashSet::new())
                        .insert(journey.id());
                });
                acc
            })
    }
}
