use std::{error::Error, rc::Rc};

use crate::{
    models::{TimetableKeyData, AttributeCollection, AttributePrimaryIndex, BitFieldCollection, BitFieldPrimaryIndex, HolidayCollection, JourneyPlatformCollection, JourneyPlatformPrimaryIndex, PlatformCollection, PlatformPrimaryIndex, StopCollection, StopPrimaryIndex, ThroughServiceCollection, DirectionPrimaryIndex, DirectionCollection},
    parsing,
};

#[allow(unused)]
#[derive(Debug)]
pub struct Hrdf {
    attributes: AttributeCollection,
    attributes_primary_index: AttributePrimaryIndex, // Key = Attribute.id

    bit_fields: BitFieldCollection,
    bit_fields_primary_index: BitFieldPrimaryIndex, // Key = BitField.id

    directions: DirectionCollection,
    directions_primary_index: DirectionPrimaryIndex, // Key Direction.id

    holidays: HolidayCollection,

    journey_platform: JourneyPlatformCollection,
    journey_platform_primary_index: JourneyPlatformPrimaryIndex, // Key = (Stop.id, Platform.id)

    platforms: PlatformCollection,
    platforms_primary_index: PlatformPrimaryIndex, // Key = Platform.id

    stops: StopCollection,
    stops_primary_index: StopPrimaryIndex, // Key = Stop.id

    through_services: ThroughServiceCollection,

    timetable_key_data: TimetableKeyData,
}

#[allow(unused)]
impl Hrdf {
    pub fn new() -> Result<Rc<Self>, Box<dyn Error>> {
        let (attributes, attributes_primary_index) = parsing::load_attributes()?;
        let (bit_fields, bit_fields_primary_index) = parsing::load_bit_fields()?;
        let (directions, directions_primary_index) = parsing::load_directions()?;
        let holidays = parsing::load_holidays()?;
        let (journey_platform, journey_platform_primary_index, platforms, platforms_primary_index) =
            parsing::load_journey_platform_and_platforms()?;
        let (stops, stops_primary_index) = parsing::load_stops()?;
        let through_services = parsing::load_through_services()?;
        let timetable_key_data = parsing::load_timetable_key_data()?;

        let instance = Rc::new(Self {
            attributes,
            bit_fields,
            bit_fields_primary_index,
            directions,
            directions_primary_index,
            holidays,
            attributes_primary_index,
            journey_platform,
            journey_platform_primary_index,
            platforms,
            platforms_primary_index,
            stops,
            stops_primary_index,
            through_services,
            timetable_key_data,
        });

        // Self::set_parent_references(&instance);
        Ok(instance)
    }

    // fn set_parent_references(instance: &Rc<Hrdf>) {
    //     for stop in &instance.stops {
    //         stop.set_parent_reference(&instance);
    //     }
    // }

    pub fn platforms(&self) -> &PlatformCollection {
        return &self.platforms;
    }

    pub fn stops(&self) -> &StopCollection {
        &self.stops
    }

    pub fn stops_primary_index(&self) -> &StopPrimaryIndex {
        &self.stops_primary_index
    }
}
