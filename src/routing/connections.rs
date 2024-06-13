use std::collections::HashSet;

use chrono::{Duration, NaiveDateTime, NaiveTime};

use crate::{
    models::{Journey, Model},
    storage::DataStorage,
    utils::add_1_day,
};

use super::{
    models::{Route, RouteSection},
    utils::{get_nearby_stops, get_operating_journeys, get_routes_to_ignore, sort_routes},
};

pub fn create_initial_routes(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    target_arrival_stop_id: i32,
    departure_at: NaiveDateTime,
) -> Vec<Route> {
    let mut routes: Vec<Route> =
        next_departures(data_storage, departure_stop_id, departure_at, None)
            .iter()
            .filter_map(|(journey, journey_departure_at)| {
                get_next_route_section(
                    data_storage,
                    journey,
                    departure_stop_id,
                    *journey_departure_at,
                    target_arrival_stop_id,
                    false,
                )
                .map(|(route_section, mut visited_stops)| {
                    visited_stops.insert(departure_stop_id);
                    Route::new(vec![route_section], visited_stops)
                })
            })
            .collect();
    get_nearby_stops(data_storage, departure_stop_id).map(|stop_connections| {
        routes.extend(stop_connections.iter().map(|stop_connection| {
            let mut visited_stops = HashSet::new();
            visited_stops.insert(stop_connection.stop_id_1());
            visited_stops.insert(stop_connection.stop_id_2());

            let route_section = RouteSection::new(
                None,
                stop_connection.stop_id_1(),
                stop_connection.stop_id_2(),
                departure_at,
                Some(stop_connection.duration()),
            );

            Route::new(vec![route_section], visited_stops)
        }));
    });
    sort_routes(&mut routes);
    routes
}

pub fn get_connections(
    data_storage: &DataStorage,
    route: &Route,
    target_arrival_stop_id: i32,
) -> Vec<Route> {
    next_departures(
        data_storage,
        route.arrival_stop_id(),
        route.arrival_at(),
        Some(get_routes_to_ignore(data_storage, &route)),
    )
    .iter()
    .filter_map(|(journey, departure_at)| {
        create_route_from_another_route(
            data_storage,
            &route,
            journey.id(),
            route.last_route_section().arrival_stop_id(),
            *departure_at,
            target_arrival_stop_id,
        )
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

    // Pas incroyable :

    let journeys_1: Vec<(&Journey, NaiveDateTime)> =
        get_operating_journeys(data_storage, departure_date_1, departure_stop_id)
            .into_iter()
            .filter(|journey| !journey.is_last_stop(departure_stop_id))
            .map(|journey| {
                let journey_departure_time: NaiveTime =
                    journey.departure_time_of(departure_stop_id).into();
                let journey_departure_at =
                    NaiveDateTime::new(departure_date_1, journey_departure_time);

                (journey, journey_departure_at)
            })
            .collect();

    let journeys_2: Vec<(&Journey, NaiveDateTime)> =
        get_operating_journeys(data_storage, departure_date_2, departure_stop_id)
            .into_iter()
            .filter(|journey| !journey.is_last_stop(departure_stop_id))
            .map(|journey| {
                let journey_departure_time: NaiveTime =
                    journey.departure_time_of(departure_stop_id).into();
                let journey_departure_at =
                    NaiveDateTime::new(departure_date_2, journey_departure_time);

                (journey, journey_departure_at)
            })
            .collect();

    let max_departure_at = departure_at.checked_add_signed(Duration::hours(4)).unwrap();
    let mut journeys: Vec<(&Journey, NaiveDateTime)> = [journeys_1, journeys_2]
        .concat()
        .into_iter()
        .filter(|(_, journey_departure_at)| {
            *journey_departure_at >= departure_at && *journey_departure_at <= max_departure_at
        })
        .collect();

    journeys.sort_by(|(_, a), (_, b)| a.cmp(b));

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
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
    target_arrival_stop_id: i32,
) -> Option<Route> {
    let is_same_journey = route
        .last_route_section()
        .journey_id()
        .map_or(false, |j| j == journey_id);

    get_next_route_section(
        data_storage,
        data_storage.journeys().find(journey_id),
        departure_stop_id,
        departure_at,
        target_arrival_stop_id,
        is_same_journey,
    )
    .and_then(|(new_route_section, new_visited_stops)| {
        if route.has_visited_any_stops(&new_visited_stops) {
            return None;
        }

        let mut cloned_route_sections: Vec<RouteSection> = route.route_sections().clone();
        let mut cloned_visited_stops = route.visited_stops().clone();

        if is_same_journey {
            let last_route_section = cloned_route_sections.last_mut().unwrap();
            last_route_section.set_arrival_stop_id(new_route_section.arrival_stop_id());
            last_route_section.set_arrival_at(new_route_section.arrival_at());
        } else {
            cloned_route_sections.push(new_route_section);
        }

        cloned_visited_stops.extend(new_visited_stops);

        Some(Route::new(cloned_route_sections, cloned_visited_stops))
    })
}

pub fn get_next_route_section(
    data_storage: &DataStorage,
    journey: &Journey,
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
    target_arrival_stop_id: i32,
    skip_first_route_entry: bool,
) -> Option<(RouteSection, HashSet<i32>)> {
    let mut route_iter = journey.route().iter().peekable();

    if skip_first_route_entry {
        route_iter.next();
    }

    while route_iter.peek().unwrap().stop_id() != departure_stop_id {
        route_iter.next();
    }

    route_iter.next();
    let mut visited_stops = HashSet::new();

    while route_iter.peek().is_some() {
        let stop = data_storage
            .stops()
            .find(route_iter.peek().unwrap().stop_id());
        visited_stops.insert(stop.id());

        if stop.transfer_flag() != 0 || stop.id() == target_arrival_stop_id {
            let arrival_time: NaiveTime = journey.arrival_time_of(stop.id()).into();

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

        route_iter.next();
    }

    None
}

pub fn get_nearby_stop_connections(data_storage: &DataStorage, route: &Route) -> Vec<Route> {
    let stop_connections = get_nearby_stops(data_storage, route.arrival_stop_id());

    if stop_connections.is_none() {
        return Vec::new();
    }

    stop_connections
        .unwrap()
        .into_iter()
        .filter(|stop_connection| !route.visited_stops().contains(&stop_connection.stop_id_2()))
        .map(|stop_connection| {
            let mut cloned_route_sections = route.route_sections().clone();
            let mut cloned_visited_stops = route.visited_stops().clone();

            cloned_route_sections.push(RouteSection::new(
                None,
                stop_connection.stop_id_1(),
                stop_connection.stop_id_2(),
                route
                    .arrival_at()
                    .checked_add_signed(Duration::minutes(stop_connection.duration().into()))
                    .unwrap(),
                Some(stop_connection.duration()),
            ));
            cloned_visited_stops.insert(stop_connection.stop_id_2());

            Route::new(cloned_route_sections, cloned_visited_stops)
        })
        .collect()
}
