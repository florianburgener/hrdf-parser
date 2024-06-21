use chrono::{Duration, NaiveDate, NaiveDateTime};
use rustc_hash::FxHashSet;

use crate::{
    models::{Journey, Model},
    storage::DataStorage,
    utils::add_1_day,
};

use super::{
    constants::MAXIMUM_NUMBER_OF_HOURS_TO_CHECK_FOR_NEXT_DEPARTURES, models::Route,
    utils::get_routes_to_ignore,
};

pub fn get_connections(
    data_storage: &DataStorage,
    route: &Route,
    journeys_to_ignore: &FxHashSet<i32>,
) -> Vec<Route> {
    next_departures(
        data_storage,
        route.arrival_stop_id(),
        route.arrival_at(),
        Some(get_routes_to_ignore(data_storage, &route)),
    )
    .into_iter()
    .filter(|(journey, _)| !journeys_to_ignore.contains(&journey.id()))
    .filter_map(|(journey, journey_departure_at)| {
        route.extend(data_storage, journey.id(), journey_departure_at)
    })
    .collect()
}

pub fn next_departures<'a>(
    data_storage: &'a DataStorage,
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
    routes_to_ignore: Option<FxHashSet<u64>>,
) -> Vec<(&'a Journey, NaiveDateTime)> {
    let departure_date_1 = departure_at.date();
    let departure_date_2 = add_1_day(departure_at.date());

    let journeys_1: Vec<(&Journey, NaiveDateTime)> =
        get_operating_journeys(data_storage, departure_date_1, departure_stop_id)
            .into_iter()
            .filter(|journey| !journey.is_last_stop(departure_stop_id, true))
            .map(|journey| {
                let journey_departure_time = journey.departure_time_of(departure_stop_id);
                let journey_departure_at =
                    NaiveDateTime::new(departure_date_1, journey_departure_time);

                (journey, journey_departure_at)
            })
            .collect();

    let journeys_2: Vec<(&Journey, NaiveDateTime)> =
        get_operating_journeys(data_storage, departure_date_2, departure_stop_id)
            .into_iter()
            .filter(|journey| !journey.is_last_stop(departure_stop_id, true))
            .map(|journey| {
                let journey_departure_time = journey.departure_time_of(departure_stop_id);
                let journey_departure_at =
                    NaiveDateTime::new(departure_date_2, journey_departure_time);

                (journey, journey_departure_at)
            })
            .collect();

    let max_departure_at = departure_at
        .checked_add_signed(Duration::hours(
            MAXIMUM_NUMBER_OF_HOURS_TO_CHECK_FOR_NEXT_DEPARTURES,
        ))
        .unwrap();
    let mut journeys: Vec<(&Journey, NaiveDateTime)> = [journeys_1, journeys_2]
        .concat()
        .into_iter()
        .filter(|&(_, journey_departure_at)| {
            journey_departure_at >= departure_at && journey_departure_at <= max_departure_at
        })
        .collect();

    journeys.sort_by_key(|(_, journey_departure_at)| *journey_departure_at);

    let mut routes_to_ignore = routes_to_ignore.unwrap_or_else(FxHashSet::default);

    journeys
        .into_iter()
        .filter(|(journey, _)| {
            let hash = journey.hash_route(departure_stop_id).unwrap();

            if !routes_to_ignore.contains(&hash) {
                routes_to_ignore.insert(hash);
                true
            } else {
                false
            }
        })
        .collect()
}

pub fn get_operating_journeys(
    data_storage: &DataStorage,
    date: NaiveDate,
    stop_id: i32,
) -> Vec<&Journey> {
    let bit_fields_1 = data_storage.bit_fields().find_by_day(date);

    data_storage
        .bit_fields()
        .find_by_stop_id(stop_id)
        .map_or(Vec::new(), |bit_fields_2| {
            let bit_fields: Vec<_> = bit_fields_1.intersection(&bit_fields_2).collect();

            bit_fields
                .into_iter()
                .map(|&bit_field_id| {
                    data_storage
                        .journeys()
                        .find_by_stop_id_and_bit_field_id(stop_id, bit_field_id)
                })
                .flatten()
                .map(|&journey_id| data_storage.journeys().find(journey_id))
                .collect()
        })
}
