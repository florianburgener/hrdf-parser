use std::{error::Error, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    models::{
        Attribute, BitField, Direction, Holiday, InformationText, JourneyPlatform, Line, Platform,
        Stop, StopConnection, ThroughService, TransferTimeAdministration, TransferTimeJourney,
        TransferTimeLine, TransportCompany, TransportType,
    },
    parsing,
    storage::{JourneyStorage, SimpleResourceStorage, TimetableMetadataStorage},
};

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Hrdf {
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
impl Hrdf {
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
        let (journey_platform, platforms) = parsing::load_platforms()?;
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

    fn build_indexes(&self) {
        self.timetable_metadata().set_find_by_key_index(self);
        self.journeys().set_operating_journeys_index(self);
    }

    pub fn set_references(&self, instance: &Rc<Hrdf>) {
        self.journeys
            .rows()
            .iter()
            .for_each(|item| item.set_hrdf(instance));
    }

    pub fn remove_references(&self) {
        self.journeys
            .rows()
            .iter()
            .for_each(|item| item.remove_hrdf());
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
