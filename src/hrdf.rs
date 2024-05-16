use std::{error::Error, rc::Rc};

use crate::{
    models::{
        AdministrationTransferTime, Attribute, BitField, Direction, Holiday, InformationText,
        Journey, JourneyPlatform, Line, Platform, Stop, StopConnection, ThroughService,
        TimetableMetadataEntry, TransportCompany, TransportType,
    },
    parsing,
    storage::SimpleResourceStorage,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    // Time-relevant data.
    bit_field: SimpleResourceStorage<BitField>,
    holiday: SimpleResourceStorage<Holiday>,
    timetable_metadata: SimpleResourceStorage<TimetableMetadataEntry>,

    // Master data.
    attribute: SimpleResourceStorage<Attribute>,
    information_text: SimpleResourceStorage<InformationText>,
    direction: SimpleResourceStorage<Direction>,
    line: SimpleResourceStorage<Line>,
    transport_company: SimpleResourceStorage<TransportCompany>,
    transport_type: SimpleResourceStorage<TransportType>,

    // Stop data.
    stop: SimpleResourceStorage<Stop>,
    stop_connection: SimpleResourceStorage<StopConnection>,

    // Transfer times.
    administration_transfer_time: SimpleResourceStorage<AdministrationTransferTime>,

    // Timetable data.
    journey: SimpleResourceStorage<Journey>,
    journey_platform: SimpleResourceStorage<JourneyPlatform>,
    platform: SimpleResourceStorage<Platform>,
    through_service: SimpleResourceStorage<ThroughService>,
}

#[allow(unused)]
impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        // Time-relevant data.
        let bit_field = parsing::load_bit_field_resource()?;
        let holiday = parsing::load_holiday_resource()?;
        let timetable_metadata = parsing::load_timetable_metadata_resource()?;

        // Master data.
        let (attribute, attribute_legacy_primary_index) = parsing::load_attribute_resource()?;
        let (direction, _) = parsing::load_direction_resource()?;
        let information_text = parsing::load_information_text_resource()?;
        let line = parsing::load_line_resource()?;
        let transport_company = parsing::load_transport_company_resource()?;
        let (transport_type, transport_type_legacy_primary_index) =
            parsing::load_transport_type_resource()?;

        // Stop data.
        let stop = parsing::load_stop_resource()?;
        let stop_connection =
            parsing::load_stop_connection_resource(attribute_legacy_primary_index)?;

        // Transfer times.
        let administration_transfer_time = parsing::load_administration_transfer_time_resource()?;

        // Timetable data.
        let (journey, _) = parsing::load_journey_resource(transport_type_legacy_primary_index)?;
        let (journey_platform, platform, _) = parsing::load_platform_resource()?;
        let through_service = parsing::load_through_service_resource()?;

        let instance = Rc::new(Self {
            // Time-relevant data.
            bit_field,
            holiday,
            timetable_metadata,
            // Master data.
            attribute,
            information_text,
            direction,
            line,
            transport_company,
            transport_type,
            // Stop data.
            stop,
            stop_connection,
            // Transfer times.
            administration_transfer_time,
            // Timetable data.
            journey,
            journey_platform,
            platform,
            through_service,
        });

        // Self::set_parent_references(&instance);
        Ok(instance)
    }

    // fn set_parent_references(instance: &Rc<Hrdf>) {
    //     for stop in &instance.stops {
    //         stop.set_parent_reference(&instance);
    //     }
    // }

    pub fn platform(&self) -> &SimpleResourceStorage<Platform> {
        return &self.platform;
    }

    pub fn stop(&self) -> &SimpleResourceStorage<Stop> {
        &self.stop
    }
}
