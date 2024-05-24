use std::{
    cell::{Ref, RefCell},
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

use chrono::{Days, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        Attribute, BitField, Direction, Holiday, InformationText, JourneyPlatform, Line, Platform,
        Stop, StopConnection, ThroughService, TransferTimeAdministration, TransferTimeJourney,
        TransferTimeLine, TransportCompany, TransportType,
    },
    parsing,
};
use crate::{
    models::{Journey, Model, ResourceCollection, ResourceIndex, TimetableMetadataEntry},
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
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        // Time-relevant data.
        let bit_fields = parsing::load_bit_fields()?;
        let holidays = parsing::load_holidays()?;
        let timetable_metadata = parsing::load_timetable_metadata()?;

        // Master data.
        let (attributes, attributes_original_primary_index) = parsing::load_attributes()?;
        let (directions, directions_original_primary_index) = parsing::load_directions()?;
        let information_texts = parsing::load_information_texts()?;
        let lines = parsing::load_lines()?;
        let transport_companies = parsing::load_transport_companies()?;
        let (transport_types, transport_types_original_primary_index) =
            parsing::load_transport_types()?;

        // Stop data.
        let stops = parsing::load_stops()?;
        let stop_connections = parsing::load_stop_connections(&attributes_original_primary_index)?;

        // Timetable data.
        let (journeys, journeys_original_primary_index) = parsing::load_journeys(
            &transport_types_original_primary_index,
            &attributes_original_primary_index,
            &directions_original_primary_index,
        )?;
        let (journey_platform, platforms) =
            parsing::load_platforms(&journeys_original_primary_index)?;
        let through_service = parsing::load_through_service(&journeys_original_primary_index)?;

        // Transfer times.
        let transfer_times_administration = parsing::load_transfer_times_administration()?;
        let transfer_times_journey =
            parsing::load_transfer_times_journey(&journeys_original_primary_index)?;
        let transfer_times_line =
            parsing::load_transfer_times_line(&transport_types_original_primary_index)?;

        let instance = Rc::new(Self {
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
        });

        instance.set_references(&instance);
        instance.build_indexes();
        Ok(instance)
    }

    pub fn set_references(&self, instance: &Rc<DataStorage>) {
        self.journeys
            .rows()
            .iter()
            .for_each(|item| item.set_data_storage_reference(instance));
    }

    pub fn remove_references(&self) {
        self.journeys
            .rows()
            .iter()
            .for_each(|item| item.remove_data_storage_reference());
    }

    fn build_indexes(&self) {
        self.journeys().build_journeys_by_day(self);
    }

    pub fn bit_fields(&self) -> &SimpleResourceStorage<BitField> {
        return &self.bit_fields;
    }

    pub fn timetable_metadata(&self) -> &TimetableMetadataStorage {
        return &self.timetable_metadata;
    }

    pub fn journeys(&self) -> &JourneyStorage {
        return &self.journeys;
    }

    pub fn platforms(&self) -> &SimpleResourceStorage<Platform> {
        return &self.platforms;
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
    rows: ResourceCollection<M>,
    primary_index: ResourceIndex<M::K, M>,
}

#[allow(unused)]
impl<M: Model<M>> SimpleResourceStorage<M> {
    pub fn new(rows: ResourceCollection<M>) -> Self {
        let primary_index = M::create_primary_index(&rows);

        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &ResourceCollection<M> {
        &self.rows
    }

    pub fn primary_index(&self) -> &ResourceIndex<M::K, M> {
        &self.primary_index
    }

    pub fn find(&self, k: M::K) -> Rc<M> {
        Rc::clone(self.primary_index().get(&k).unwrap())
    }
}

// ------------------------------------------------------------------------------------------------
// --- TimetableMetadataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct TimetableMetadataStorage {
    rows: ResourceCollection<TimetableMetadataEntry>,
    primary_index: ResourceIndex<i32, TimetableMetadataEntry>,
    timetable_metadata_entry_by_key: ResourceIndex<String, TimetableMetadataEntry>,
}

#[allow(unused)]
impl TimetableMetadataStorage {
    pub fn new(rows: ResourceCollection<TimetableMetadataEntry>) -> Self {
        let primary_index = TimetableMetadataEntry::create_primary_index(&rows);
        let timetable_metadata_entry_by_key = Self::create_timetable_metadata_entry_by_key(&rows);

        Self {
            rows,
            primary_index,
            timetable_metadata_entry_by_key,
        }
    }

    fn create_timetable_metadata_entry_by_key(
        rows: &ResourceCollection<TimetableMetadataEntry>,
    ) -> ResourceIndex<String, TimetableMetadataEntry> {
        rows.iter().fold(HashMap::new(), |mut acc, item| {
            acc.insert(item.key().to_owned(), Rc::clone(&item));
            acc
        })
    }

    pub fn rows(&self) -> &ResourceCollection<TimetableMetadataEntry> {
        &self.rows
    }

    pub fn primary_index(&self) -> &ResourceIndex<i32, TimetableMetadataEntry> {
        &self.primary_index
    }

    pub fn find_by_key(&self, k: &str) -> Rc<TimetableMetadataEntry> {
        Rc::clone(self.timetable_metadata_entry_by_key.get(k).unwrap())
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
    rows: ResourceCollection<Journey>,
    primary_index: ResourceIndex<i32, Journey>,
    journeys_by_day: RefCell<HashMap<NaiveDate, HashSet<i32>>>,
    journeys_by_stop_id: HashMap<i32, HashSet<i32>>,
}

#[allow(unused)]
impl JourneyStorage {
    pub fn new(rows: ResourceCollection<Journey>) -> Self {
        let primary_index = Journey::create_primary_index(&rows);
        let journeys_by_stop_id = Self::create_journeys_by_stop_id(&rows);

        Self {
            rows,
            primary_index,
            journeys_by_day: RefCell::new(HashMap::default()),
            journeys_by_stop_id,
        }
    }

    fn create_journeys_by_stop_id(
        rows: &ResourceCollection<Journey>,
    ) -> HashMap<i32, HashSet<i32>> {
        rows.iter().fold(HashMap::new(), |mut acc, journey| {
            journey.route().iter().for_each(|route_entry| {
                acc.entry(route_entry.stop_id())
                    .or_insert(HashSet::new())
                    .insert(journey.id());
            });
            acc
        })
    }

    pub fn rows(&self) -> &ResourceCollection<Journey> {
        &self.rows
    }

    pub fn primary_index(&self) -> &ResourceIndex<i32, Journey> {
        &self.primary_index
    }

    pub fn build_journeys_by_day(&self, data_storage: &DataStorage) {
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

        *self.journeys_by_day.borrow_mut() =
            self.rows().iter().fold(HashMap::new(), |mut acc, item| {
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
            });
    }

    pub fn journeys_by_day(&self) -> Ref<HashMap<NaiveDate, HashSet<i32>>> {
        self.journeys_by_day.borrow()
    }

    pub fn find_journeys_for_specific_day(&self, day: NaiveDate) -> HashSet<i32> {
        self.journeys_by_day
            .borrow()
            .get(&day)
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }

    pub fn find_journeys_for_specific_stop_id(&self, stop_id: i32) -> &HashSet<i32> {
        self.journeys_by_stop_id.get(&stop_id).unwrap()
    }

    pub fn find(&self, k: i32) -> Rc<Journey> {
        Rc::clone(self.primary_index().get(&k).unwrap())
    }

    pub fn get(&self, ids: HashSet<i32>) -> ResourceCollection<Journey> {
        ids.into_iter().map(|id| self.find(id)).collect()
    }
}
