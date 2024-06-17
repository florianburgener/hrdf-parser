use std::collections::HashSet;

use chrono::{Duration, NaiveDateTime};

use crate::{
    models::{Journey, Model},
    storage::DataStorage,
    utils::add_1_day,
};

use super::{
    constants::MAXIMUM_NUMBER_OF_HOURS_TO_CHECK_FOR_NEXT_DEPARTURES,
    models::{Route, RouteSection},
    utils::{
        clone_update_route, get_operating_journeys, get_routes_to_ignore, get_stop_connections,
        sort_routes,
    },
};

pub fn create_initial_routes(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
) -> Vec<Route> {
    let mut routes: Vec<Route> =
        next_departures(data_storage, departure_stop_id, departure_at, None)
            .into_iter()
            .filter_map(|(journey, journey_departure_at)| {
                get_next_route_section(
                    data_storage,
                    journey,
                    departure_stop_id,
                    journey_departure_at,
                )
                .map(|(section, mut visited_stops)| {
                    visited_stops.insert(departure_stop_id);
                    Route::new(vec![section], visited_stops)
                })
            })
            .collect();

    if let Some(stop_connections) = get_stop_connections(data_storage, departure_stop_id) {
        routes.extend(stop_connections.iter().map(|stop_connection| {
            let mut visited_stops = HashSet::new();
            visited_stops.insert(stop_connection.stop_id_1());
            visited_stops.insert(stop_connection.stop_id_2());

            let section = RouteSection::new(
                None,
                stop_connection.stop_id_1(),
                stop_connection.stop_id_2(),
                departure_at,
                Some(stop_connection.duration()),
            );

            Route::new(vec![section], visited_stops)
        }));
    }

    sort_routes(&mut routes);
    routes
}

pub fn get_connections(data_storage: &DataStorage, route: &Route) -> Vec<Route> {
    next_departures(
        data_storage,
        route.arrival_stop_id(),
        route.arrival_at(),
        Some(get_routes_to_ignore(data_storage, &route)),
    )
    .iter()
    .filter_map(|(journey, journey_departure_at)| {
        create_route_from_another_route(data_storage, &route, journey.id(), *journey_departure_at)
    })
    .collect()
}

pub fn next_departures<'a>(
    data_storage: &'a DataStorage,
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
    routes_to_ignore: Option<HashSet<u64>>,
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
        .filter(|(_, journey_departure_at)| {
            *journey_departure_at >= departure_at && *journey_departure_at <= max_departure_at
        })
        .collect();

    journeys.sort_by_key(|(_, journey_departure_at)| *journey_departure_at);

    let mut routes_to_ignore = routes_to_ignore.unwrap_or_else(HashSet::new);

    journeys
        .into_iter()
        .filter(|(journey, _)| {
            let hash = journey.hash_route(departure_stop_id).unwrap();
            let contains = routes_to_ignore.contains(&hash);

            if !contains {
                routes_to_ignore.insert(hash);
            }

            !contains
        })
        .collect()
}

pub fn create_route_from_another_route(
    data_storage: &DataStorage,
    route: &Route,
    journey_id: i32,
    departure_at: NaiveDateTime,
) -> Option<Route> {
    let journey = data_storage.journeys().find(journey_id);

    if journey.is_last_stop(route.last_section().arrival_stop_id(), false) {
        return None;
    }

    let is_same_journey = route
        .last_section()
        .journey_id()
        .map_or(false, |id| id == journey_id);

    get_next_route_section(
        data_storage,
        journey,
        route.last_section().arrival_stop_id(),
        departure_at,
    )
    .and_then(|(new_section, new_visited_stops)| {
        if route.has_visited_any_stops(&new_visited_stops)
            && new_section.arrival_stop_id() != journey.first_stop_id()
        {
            return None;
        }

        let new_route = clone_update_route(route, |cloned_sections, cloned_visited_stops| {
            if is_same_journey {
                let last_section = cloned_sections.last_mut().unwrap();
                last_section.set_arrival_stop_id(new_section.arrival_stop_id());
                last_section.set_arrival_at(new_section.arrival_at());
            } else {
                cloned_sections.push(new_section);
            }

            cloned_visited_stops.extend(new_visited_stops);
        });
        Some(new_route)
    })
}

fn get_next_route_section(
    data_storage: &DataStorage,
    journey: &Journey,
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
) -> Option<(RouteSection, HashSet<i32>)> {
    let mut route_iter = journey.route().iter();

    while let Some(route_entry) = route_iter.next() {
        if route_entry.stop_id() == departure_stop_id {
            break;
        }
    }

    let mut visited_stops = HashSet::new();

    while let Some(route_entry) = route_iter.next() {
        let stop = data_storage.stops().find(route_entry.stop_id());
        visited_stops.insert(stop.id());

        if stop.can_be_used_as_exchange_point() || journey.is_last_stop(stop.id(), false) {
            let arrival_time = journey.arrival_time_of(stop.id());

            let arrival_at = if arrival_time >= departure_at.time() {
                NaiveDateTime::new(departure_at.date(), arrival_time)
            } else {
                NaiveDateTime::new(add_1_day(departure_at.date()), arrival_time)
            };

            return Some((
                RouteSection::new(
                    Some(journey.id()),
                    departure_stop_id,
                    stop.id(),
                    arrival_at,
                    None,
                ),
                visited_stops,
            ));
        }
    }

    None
}
