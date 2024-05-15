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
    attribute: SimpleResourceStorage<Attribute>,
    bit_field: SimpleResourceStorage<BitField>,
    direction: SimpleResourceStorage<Direction>,
    holiday: SimpleResourceStorage<Holiday>,
    information_text: SimpleResourceStorage<InformationText>,
    line: SimpleResourceStorage<Line>,
    journey_platform: SimpleResourceStorage<JourneyPlatform>,
    platform: SimpleResourceStorage<Platform>,
    stop: SimpleResourceStorage<Stop>,
    stop_connection: SimpleResourceStorage<StopConnection>,
    through_service: SimpleResourceStorage<ThroughService>,
    timetable_metadata: SimpleResourceStorage<TimetableMetadata>,
    transport_company: SimpleResourceStorage<TransportCompany>,
    transport_type: SimpleResourceStorage<TransportType>,
}

#[allow(unused)]
impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let (attribute, _) = parsing::load_attribute_resource()?;
        let bit_field = parsing::load_bit_field_resource()?;
        let (direction, _) = parsing::load_direction_resource()?;
        let holiday = parsing::load_holiday_resource()?;
        let information_text = parsing::load_information_text_resource()?;
        let line = parsing::load_line_resource()?;
        let (journey_platform, platform, _) = parsing::load_platform_resource()?;
        let stop = parsing::load_stop_resource()?;
        let stop_connection = parsing::load_stop_connection_resource()?;
        let through_service = parsing::load_through_service_resource()?;
        let timetable_metadata = parsing::load_timetable_metadata_resource()?;
        let transport_company = parsing::load_transport_company_resource()?;
        let (transport_type, _) = parsing::load_transport_type_resource()?;

        let instance = Rc::new(Self {
            attribute,
            bit_field,
            direction,
            holiday,
            information_text,
            line,
            journey_platform,
            platform,
            stop,
            stop_connection,
            through_service,
            timetable_metadata,
            transport_company,
            transport_type,
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
