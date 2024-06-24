use chrono::{Duration, NaiveDate, NaiveDateTime};
use rustc_hash::FxHashSet;

use crate::{
    models::{Journey, Model},
    storage::DataStorage,
    utils::{add_1_day, create_time},
};

use super::{models::Route, utils::get_routes_to_ignore};

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
        route.last_section().journey_id(),
    )
    .into_iter()
    .filter(|(journey, _)| !journeys_to_ignore.contains(&journey.id()))
    .filter_map(|(journey, journey_departure_at)| {
        route.extend(
            data_storage,
            journey.id(),
            journey_departure_at.date(),
            true,
        )
    })
    .collect()
}

pub fn next_departures<'a>(
    data_storage: &'a DataStorage,
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
    routes_to_ignore: Option<FxHashSet<u64>>,
    _previous_journey_id: Option<i32>,
) -> Vec<(&'a Journey, NaiveDateTime)> {
    fn get_journeys(
        data_storage: &DataStorage,
        date: NaiveDate,
        stop_id: i32,
    ) -> Vec<(&Journey, NaiveDateTime)> {
        get_operating_journeys(data_storage, date, stop_id)
            .into_iter()
            .filter(|journey| !journey.is_last_stop(stop_id, true))
            .map(|journey| {
                let journey_departure_at = journey.departure_at_of(stop_id, date);
                // if journey.id() == 105975 && stop_id == 8503227 && journey_departure_at.date() == create_date(2023, month, day) {
                //     println!("{} {} \n\n{:?}", date, journey_departure_at, journey.route());
                //     panic!();
                // }
                (journey, journey_departure_at)
            })
            .collect()
    }

    let journeys_1: Vec<(&Journey, NaiveDateTime)> =
        get_journeys(data_storage, departure_at.date(), departure_stop_id);

    let (journeys_2, max_departure_at) = if departure_at.time() >= create_time(18, 0) {
        // The journeys of the next day are also loaded.
        // The maximum departure time is 08:00 the next day.
        let departure_date = add_1_day(departure_at.date());
        let journeys: Vec<(&Journey, NaiveDateTime)> =
            get_journeys(data_storage, departure_date, departure_stop_id);
        let max_departure_at = NaiveDateTime::new(departure_date, create_time(8, 0));

        (journeys, max_departure_at)
    } else {
        // The next day's journeys are not loaded.
        let max_departure_at = if departure_at.time() < create_time(8, 0) {
            // The maximum departure time is 08:00.
            NaiveDateTime::new(departure_at.date(), create_time(8, 0))
        } else {
            // The maximum departure time is 4 hours later.
            departure_at.checked_add_signed(Duration::hours(4)).unwrap()
        };

        (Vec::new(), max_departure_at)
    };

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
        // .filter(|&(journey, journey_departure_at)| {
        //     previous_journey_id.map_or(true, |id| {
        //         add_minutes_to_date_time(
        //             departure_at,
        //             exchange_time(data_storage, departure_stop_id, id, journey.id()) as i64,
        //         ) <= journey_departure_at
        //     })
        // })
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

// pub fn exchange_time(
//     data_storage: &DataStorage,
//     stop_id: i32,
//     journey_id_1: i32,
//     journey_id_2: i32,
// ) -> i16 {
//     2
// }
