use std::{error::Error, rc::Rc};

use crate::{
    models::{
        Attribute, BitField, Direction, Holiday, InformationText, JourneyPlatform, Line, Platform,
        Stop, StopConnection, ThroughService, TimetableMetadata, TransportCompany, TransportType,
    },
    parsing,
    storage::SimpleResourceStorage,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    attributes: SimpleResourceStorage<Attribute>,
    bit_fields: SimpleResourceStorage<BitField>,
    directions: SimpleResourceStorage<Direction>,
    holidays: SimpleResourceStorage<Holiday>,
    information_texts: SimpleResourceStorage<InformationText>,
    lines: SimpleResourceStorage<Line>,
    journey_platform: SimpleResourceStorage<JourneyPlatform>,
    platforms: SimpleResourceStorage<Platform>,
    stop_connections: SimpleResourceStorage<StopConnection>,
    stops: SimpleResourceStorage<Stop>,
    through_service_entries: SimpleResourceStorage<ThroughService>,
    timetable_metadata: SimpleResourceStorage<TimetableMetadata>,
    transport_companies: SimpleResourceStorage<TransportCompany>,
    transport_types: SimpleResourceStorage<TransportType>,
}

#[allow(unused)]
impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let (attributes, _) = parsing::load_attributes()?;
        let bit_fields = parsing::load_bit_fields()?;
        let (directions, _) = parsing::load_directions()?;
        let holidays = parsing::load_holidays()?;
        let information_texts = parsing::load_information_texts()?;
        let lines = parsing::load_lines()?;
        let (journey_platform, platforms, _) = parsing::load_platforms()?;
        let stop_connections = parsing::load_stop_connections()?;
        let stops = parsing::load_stops()?;
        let through_service_entries = parsing::load_through_service_entries()?;
        let timetable_metadata = parsing::load_timetable_metadata()?;
        let transport_companies = parsing::load_transport_companies()?;
        let (transport_types, _) = parsing::load_transport_types()?;

        let instance = Rc::new(Self {
            attributes,
            bit_fields,
            directions,
            holidays,
            information_texts,
            lines,
            journey_platform,
            platforms,
            stop_connections,
            stops,
            through_service_entries,
            timetable_metadata,
            transport_companies,
            transport_types,
        });

        // Self::set_parent_references(&instance);
        Ok(instance)
    }

    // fn set_parent_references(instance: &Rc<Hrdf>) {
    //     for stop in &instance.stops {
    //         stop.set_parent_reference(&instance);
    //     }
    // }

    pub fn platforms(&self) -> &SimpleResourceStorage<Platform> {
        return &self.platforms;
    }

    pub fn stops(&self) -> &SimpleResourceStorage<Stop> {
        &self.stops
    }
}
