use std::error::Error;

use chrono::{Days, NaiveDate};
use rustc_hash::{FxHashMap, FxHashSet};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        Attribute, BitField, Direction, ExchangeTimeAdministration, ExchangeTimeJourney,
        ExchangeTimeLine, Holiday, InformationText, Journey, JourneyPlatform, Line, Model,
        Platform, Stop, StopConnection, ThroughService, TimetableMetadataEntry, TransportCompany,
        TransportType, Version,
    },
    parsing,
    utils::count_days_between_two_dates,
};

// ------------------------------------------------------------------------------------------------
// --- DataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct DataStorage {
    // Time-relevant data.
    bit_fields: BitFieldStorage,
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
    stop_connections: StopConnectionStorage,

    // Timetable data.
    journeys: JourneyStorage,
    journey_platform: SimpleResourceStorage<JourneyPlatform>,
    platforms: SimpleResourceStorage<Platform>,
    through_service: SimpleResourceStorage<ThroughService>,

    // Exchange times.
    exchange_times_administration: SimpleResourceStorage<ExchangeTimeAdministration>,
    exchange_times_journey: SimpleResourceStorage<ExchangeTimeJourney>,
    exchange_times_line: SimpleResourceStorage<ExchangeTimeLine>,
}

#[allow(unused)]
impl DataStorage {
    pub fn new(version: Version, path: &str) -> Result<Self, Box<dyn Error>> {
        // Time-relevant data.
        let bit_fields = parsing::load_bit_fields(path)?;
        let holidays = parsing::load_holidays(path)?;
        let timetable_metadata = parsing::load_timetable_metadata(path)?;

        // Master data.
        let (attributes, attributes_pk_type_converter) = parsing::load_attributes(path)?;
        let (directions, directions_pk_type_converter) = parsing::load_directions(path)?;
        let information_texts = parsing::load_information_texts(path)?;
        let lines = parsing::load_lines(path)?;
        let transport_companies = parsing::load_transport_companies(path)?;
        let (transport_types, transport_types_pk_type_converter) =
            parsing::load_transport_types(path)?;

        // Stop data.
        let stop_connections = parsing::load_stop_connections(path, &attributes_pk_type_converter)?;
        let stops = parsing::load_stops(version, path)?;

        // Timetable data.
        let (journeys, journeys_pk_type_converter) = parsing::load_journeys(
            path,
            &transport_types_pk_type_converter,
            &attributes_pk_type_converter,
            &directions_pk_type_converter,
        )?;
        let (journey_platform, platforms) =
            parsing::load_platforms(path, &journeys_pk_type_converter)?;
        let through_service = parsing::load_through_service(path, &journeys_pk_type_converter)?;

        // Exchange times.
        let exchange_times_administration = parsing::load_exchange_times_administration(path)?;
        let exchange_times_journey =
            parsing::load_exchange_times_journey(path, &journeys_pk_type_converter)?;
        let exchange_times_line =
            parsing::load_exchange_times_line(path, &transport_types_pk_type_converter)?;

        let mut data_storage = Self {
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
            stop_connections,
            stops,
            // Timetable data.
            journeys,
            journey_platform,
            platforms,
            through_service,
            // Exchange times.
            exchange_times_administration,
            exchange_times_journey,
            exchange_times_line,
        };

        data_storage.build_indexes();
        Ok(data_storage)
    }

    fn build_indexes(&mut self) {
        let indexes = self.create_indexes();
        self.set_indexes(indexes);
    }

    fn create_indexes(&self) -> (BitFieldIndexes, JourneyIndexes) {
        let bit_field_indexes = self.bit_fields.create_indexes(self);
        let journey_indexes = self.journeys.create_indexes(self);
        (bit_field_indexes, journey_indexes)
    }

    fn set_indexes(&mut self, indexes: (BitFieldIndexes, JourneyIndexes)) {
        let (bit_field_indexes, journey_indexes) = indexes;
        self.bit_fields.set_indexes(bit_field_indexes);
        self.journeys.set_indexes(journey_indexes);
    }

    pub fn bit_fields(&self) -> &BitFieldStorage {
        &self.bit_fields
    }

    pub fn journeys(&self) -> &JourneyStorage {
        &self.journeys
    }

    pub fn lines(&self) -> &SimpleResourceStorage<Line> {
        &self.lines
    }

    pub fn platforms(&self) -> &SimpleResourceStorage<Platform> {
        &self.platforms
    }

    pub fn stop_connections(&self) -> &StopConnectionStorage {
        &self.stop_connections
    }

    pub fn stops(&self) -> &SimpleResourceStorage<Stop> {
        &self.stops
    }

    pub fn timetable_metadata(&self) -> &TimetableMetadataStorage {
        &self.timetable_metadata
    }
}

// ------------------------------------------------------------------------------------------------
// --- SimpleResourceStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleResourceStorage<M: Model<M>> {
    data: FxHashMap<M::K, M>,
}

impl<M: Model<M>> SimpleResourceStorage<M> {
    pub fn new(data: FxHashMap<M::K, M>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &FxHashMap<M::K, M> {
        &self.data
    }

    pub fn find(&self, k: M::K) -> &M {
        &self.data().get(&k).unwrap()
    }

    pub fn entries(&self) -> Vec<&M> {
        self.data.values().collect()
    }
}

// ------------------------------------------------------------------------------------------------
// --- impl_Storage
// ------------------------------------------------------------------------------------------------

macro_rules! impl_Storage {
    ($s:ty, $m:ty) => {
        #[allow(unused)]
        impl $s {
            // Getters/Setters

            pub fn data(&self) -> &FxHashMap<i32, $m> {
                &self.data
            }

            // Functions

            pub fn entries(&self) -> Vec<&$m> {
                self.data.values().collect()
            }

            pub fn find(&self, id: i32) -> &$m {
                self.data.get(&id).unwrap()
            }

            pub fn resolve_ids(&self, ids: &FxHashSet<i32>) -> Vec<&$m> {
                ids.iter().map(|&id| self.find(id)).collect()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// --- BitFieldStorage
// ------------------------------------------------------------------------------------------------

type BitFieldIndex1 = FxHashMap<NaiveDate, FxHashSet<i32>>;
type BitFieldIndex2 = FxHashMap<i32, FxHashSet<i32>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct BitFieldStorage {
    data: FxHashMap<i32, BitField>,
    bit_fields_by_day: BitFieldIndex1,
    bit_fields_by_stop_id: BitFieldIndex2,
}

impl_Storage!(BitFieldStorage, BitField);

impl BitFieldStorage {
    pub fn new(data: FxHashMap<i32, BitField>) -> Self {
        Self {
            data,
            bit_fields_by_day: FxHashMap::default(),
            bit_fields_by_stop_id: FxHashMap::default(),
        }
    }

    // Getters/Setters

    fn set_bit_fields_by_day(&mut self, value: BitFieldIndex1) {
        self.bit_fields_by_day = value;
    }

    fn set_bit_fields_by_stop_id(&mut self, value: BitFieldIndex2) {
        self.bit_fields_by_stop_id = value;
    }

    // Functions

    pub fn find_by_day(&self, day: NaiveDate) -> &FxHashSet<i32> {
        self.bit_fields_by_day.get(&day).unwrap()
    }

    pub fn find_by_stop_id(&self, stop_id: i32) -> Option<&FxHashSet<i32>> {
        self.bit_fields_by_stop_id.get(&stop_id)
    }
}

type BitFieldIndexes = (BitFieldIndex1, BitFieldIndex2);

// Indexes:
impl BitFieldStorage {
    pub fn create_indexes(&self, data_storage: &DataStorage) -> BitFieldIndexes {
        (
            self.create_bit_fields_by_day(data_storage),
            self.create_bit_fields_by_stop_id(data_storage),
        )
    }

    pub fn set_indexes(&mut self, indexes: BitFieldIndexes) {
        let (bit_fields_by_day, bit_fields_by_stop_id) = indexes;

        self.set_bit_fields_by_day(bit_fields_by_day);
        self.set_bit_fields_by_stop_id(bit_fields_by_stop_id);
    }

    fn create_bit_fields_by_day(&self, data_storage: &DataStorage) -> BitFieldIndex1 {
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

        let mut map = FxHashMap::default();
        dates.iter().for_each(|date| {
            map.entry(*date).or_insert(FxHashSet::default()).insert(0);
        });

        data_storage
            .bit_fields()
            .data()
            .keys()
            .fold(map, |mut acc, bit_field_id| {
                let bit_field = data_storage.bit_fields().find(*bit_field_id);
                let indexes: Vec<usize> = bit_field
                    .bits()
                    .iter()
                    // The first two bits must be ignored.
                    .skip(2)
                    .enumerate()
                    .filter(|(i, &x)| *i < num_days && x == 1)
                    .map(|(i, _)| i)
                    .collect();

                indexes.into_iter().for_each(|i| {
                    acc.entry(dates[i])
                        .or_insert(FxHashSet::default())
                        .insert(bit_field.id());
                });

                acc
            })
    }

    fn create_bit_fields_by_stop_id(&self, data_storage: &DataStorage) -> BitFieldIndex2 {
        data_storage
            .journeys()
            .entries()
            .iter()
            .fold(FxHashMap::default(), |mut acc, journey| {
                journey.route().iter().for_each(|route_entry| {
                    acc.entry(route_entry.stop_id())
                        .or_insert(FxHashSet::default())
                        .insert(journey.bit_field_id().unwrap_or(0));
                });
                acc
            })
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyStorage
// ------------------------------------------------------------------------------------------------

type JourneyIndex1 = FxHashMap<(i32, i32), Vec<i32>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct JourneyStorage {
    data: FxHashMap<i32, Journey>,
    journeys_by_stop_id_and_bit_field_id: JourneyIndex1,
}

impl_Storage!(JourneyStorage, Journey);

impl JourneyStorage {
    pub fn new(data: FxHashMap<i32, Journey>) -> Self {
        Self {
            data,
            journeys_by_stop_id_and_bit_field_id: FxHashMap::default(),
        }
    }

    // Getters/Setters

    fn set_journeys_by_stop_id_and_bit_field_id(&mut self, value: JourneyIndex1) {
        self.journeys_by_stop_id_and_bit_field_id = value;
    }

    // Functions

    pub fn find_by_stop_id_and_bit_field_id(&self, stop_id: i32, bit_field_id: i32) -> &Vec<i32> {
        self.journeys_by_stop_id_and_bit_field_id
            .get(&(stop_id, bit_field_id))
            .unwrap()
    }
}

type JourneyIndexes = (JourneyIndex1,);

// Indexes:
impl JourneyStorage {
    pub fn create_indexes(&self, data_storage: &DataStorage) -> JourneyIndexes {
        (self.create_journeys_by_stop_id_and_bit_field_id(data_storage),)
    }

    pub fn set_indexes(&mut self, indexes: JourneyIndexes) {
        let (journeys_by_stop_id_and_bit_field_id,) = indexes;

        self.set_journeys_by_stop_id_and_bit_field_id(journeys_by_stop_id_and_bit_field_id);
    }

    fn create_journeys_by_stop_id_and_bit_field_id(&self, _: &DataStorage) -> JourneyIndex1 {
        self.entries()
            .iter()
            .fold(FxHashMap::default(), |mut acc, journey| {
                journey.route().iter().for_each(|route_entry| {
                    acc.entry((route_entry.stop_id(), journey.bit_field_id().unwrap_or(0)))
                        .or_insert(Vec::new())
                        .push(journey.id());
                });
                acc
            })
    }
}

// ------------------------------------------------------------------------------------------------
// --- StopConnectionStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct StopConnectionStorage {
    data: FxHashMap<i32, StopConnection>,
    stop_connections_by_stop_id: FxHashMap<i32, FxHashSet<i32>>,
}

impl_Storage!(StopConnectionStorage, StopConnection);

impl StopConnectionStorage {
    pub fn new(data: FxHashMap<i32, StopConnection>) -> Self {
        let stop_connections_by_stop_id = Self::create_stop_connections_by_stop_id(&data);

        Self {
            data,
            stop_connections_by_stop_id,
        }
    }

    // Functions

    pub fn find_by_stop_id(&self, stop_id: i32) -> Option<&FxHashSet<i32>> {
        self.stop_connections_by_stop_id.get(&stop_id)
    }
}

// Indexes:
impl StopConnectionStorage {
    fn create_stop_connections_by_stop_id(
        data: &FxHashMap<i32, StopConnection>,
    ) -> FxHashMap<i32, FxHashSet<i32>> {
        data.values()
            .fold(FxHashMap::default(), |mut acc, stop_connection| {
                acc.entry(stop_connection.stop_id_1())
                    .or_insert(FxHashSet::default())
                    .insert(stop_connection.id());
                acc
            })
    }
}

// ------------------------------------------------------------------------------------------------
// --- TimetableMetadataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableMetadataStorage {
    data: FxHashMap<i32, TimetableMetadataEntry>,
    timetable_metadata_entry_by_key: FxHashMap<String, i32>,
}

impl_Storage!(TimetableMetadataStorage, TimetableMetadataEntry);

impl TimetableMetadataStorage {
    pub fn new(data: FxHashMap<i32, TimetableMetadataEntry>) -> Self {
        let timetable_metadata_entry_by_key = Self::create_timetable_metadata_entry_by_key(&data);

        Self {
            data,
            timetable_metadata_entry_by_key,
        }
    }

    // Functions.

    pub fn find_by_key(&self, key: &str) -> &TimetableMetadataEntry {
        self.find(*self.timetable_metadata_entry_by_key.get(key).unwrap())
    }

    pub fn start_date(&self) -> NaiveDate {
        self.find_by_key("start_date").value_as_NaiveDate()
    }

    pub fn end_date(&self) -> NaiveDate {
        self.find_by_key("end_date").value_as_NaiveDate()
    }
}

// Indexes:
impl TimetableMetadataStorage {
    fn create_timetable_metadata_entry_by_key(
        data: &FxHashMap<i32, TimetableMetadataEntry>,
    ) -> FxHashMap<String, i32> {
        data.values().fold(FxHashMap::default(), |mut acc, item| {
            acc.insert(item.key().to_owned(), item.id());
            acc
        })
    }
}
