use std::{error::Error, rc::Rc};

use crate::{
    models::{
        Attribute, BitField, Direction, Holiday, InformationText, Line, Stop, StopConnection,
        ThroughService, TimetableMetadata, TransportCompany, TransportType,
    },
    parsing,
    storage::{JourneyPlatformData, PlatformData, SimpleDataStorage},
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    attribute_data: SimpleDataStorage<Attribute>,
    bit_field_data: SimpleDataStorage<BitField>,
    direction_data: SimpleDataStorage<Direction>,
    holiday_data: SimpleDataStorage<Holiday>,
    information_text_data: SimpleDataStorage<InformationText>,
    line_data: SimpleDataStorage<Line>,
    journey_platform_data: JourneyPlatformData,
    platform_data: PlatformData,
    stop_connection_data: SimpleDataStorage<StopConnection>,
    stop_data: SimpleDataStorage<Stop>,
    through_service_data: SimpleDataStorage<ThroughService>,
    timetable_metadata: SimpleDataStorage<TimetableMetadata>,
    transport_company_data: SimpleDataStorage<TransportCompany>,
    transport_type_data: SimpleDataStorage<TransportType>,
}

#[allow(unused)]
impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let attribute_data = parsing::load_attribute_data()?;
        let bit_field_data = parsing::load_bit_field_data()?;
        let direction_data = parsing::load_direcation_data()?;
        let holiday_data = parsing::load_holiday_data()?;
        let information_text_data = parsing::load_information_text_data()?;
        let line_data = parsing::load_line_data()?;
        let (journey_platform_data, platform_data) = parsing::load_platform_data()?;
        let stop_connection_data = parsing::load_stop_connection_data()?;
        let stop_data = parsing::load_stop_data()?;
        let through_service_data = parsing::load_through_service_data()?;
        let timetable_metadata = parsing::load_timetable_metadata()?;
        let transport_company_data = parsing::load_transport_company_data()?;
        let transport_type_data = parsing::load_transport_type_data()?;

        let instance = Rc::new(Self {
            attribute_data,
            bit_field_data,
            direction_data,
            holiday_data,
            information_text_data,
            line_data,
            journey_platform_data,
            platform_data,
            stop_connection_data,
            stop_data,
            through_service_data,
            timetable_metadata,
            transport_company_data,
            transport_type_data,
        });

        // Self::set_parent_references(&instance);
        Ok(instance)
    }

    // fn set_parent_references(instance: &Rc<Hrdf>) {
    //     for stop in &instance.stops {
    //         stop.set_parent_reference(&instance);
    //     }
    // }

    pub fn platform_data(&self) -> &PlatformData {
        return &self.platform_data;
    }

    pub fn stop_data(&self) -> &SimpleDataStorage<Stop> {
        &self.stop_data
    }
}
