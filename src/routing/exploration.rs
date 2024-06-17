use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;

use crate::{storage::DataStorage, utils::add_minutes_to_date_time};

use super::{
    connections::{create_route_from_another_route, get_connections},
    models::{Route, RouteSection},
    utils::{clone_update_route, get_stop_connections, sort_routes, sorted_insert},
};

pub fn explore_routes<F>(
    data_storage: &DataStorage,
    mut routes: Vec<Route>,
    journeys_to_ignore: &mut HashSet<i32>,
    earliest_arrival_by_stop_id: &mut HashMap<i32, NaiveDateTime>,
    mut can_continue_exploration: F,
) -> Vec<Route>
where
    F: FnMut(&Route) -> bool,
{
    let mut new_routes = Vec::new();

    while !routes.is_empty() {
        let route = routes.remove(0);

        if !can_continue_exploration(&route) {
            continue;
        }

        if route.last_section().departure_stop_id() == route.last_section().arrival_stop_id() {
            // Some journeys start and end at the same stop, so it's not possible to know whether the journey has reached its last stop.
            // The above condition, however, lets us know that the journey is about to loop.
            continue;
        }

        explore_next_section(data_storage, &route, journeys_to_ignore, &mut routes);

        if !can_explore_connections(data_storage, &route, earliest_arrival_by_stop_id) {
            continue;
        }

        explore_nearby_stops(data_storage, &route, &mut routes);
        explore_connections(data_storage, &route, &mut new_routes);
    }

    new_routes = filter_new_routes(new_routes, journeys_to_ignore);
    sort_routes(&mut new_routes);
    new_routes
}

fn explore_next_section(
    data_storage: &DataStorage,
    route: &Route,
    journeys_to_ignore: &mut HashSet<i32>,
    routes: &mut Vec<Route>,
) {
    if route.last_section().journey_id().is_none() {
        return;
    }

    let journey_id = route.last_section().journey_id().unwrap();
    journeys_to_ignore.insert(journey_id);

    let new_route = create_route_from_another_route(
        data_storage,
        &route,
        journey_id,
        route.last_section().arrival_at(),
    );

    if new_route.is_none() {
        return;
    }

    sorted_insert(routes, new_route.unwrap());
}

fn can_explore_connections(
    data_storage: &DataStorage,
    route: &Route,
    earliest_arrival_by_stop_id: &mut HashMap<i32, NaiveDateTime>,
) -> bool {
    let stop_id = route.arrival_stop_id();
    let stop = data_storage.stops().find(stop_id);

    if !stop.can_be_used_as_exchange_point() {
        // The arrival stop of the last RouteSection of a journey is not necessarily usable for exchange, hence the check.
        return false;
    }

    let arrival_at = route.arrival_at();

    if let Some(&earliest_arrival) = earliest_arrival_by_stop_id.get(&stop_id) {
        // WARNING: Consider putting the "<=" back.
        if arrival_at < earliest_arrival {
            earliest_arrival_by_stop_id.insert(stop_id, arrival_at);
            true
        } else {
            false
        }
    } else {
        earliest_arrival_by_stop_id.insert(stop_id, arrival_at);
        true
    }
}

fn explore_connections(data_storage: &DataStorage, route: &Route, new_routes: &mut Vec<Route>) {
    new_routes.extend(get_connections(data_storage, &route));
}

fn explore_nearby_stops(data_storage: &DataStorage, route: &Route, routes: &mut Vec<Route>) {
    if route.last_section().journey_id().is_none() {
        // No walking between 2 stops, after walking between 2 stops just before.
        return;
    }

    match get_stop_connections(data_storage, route.arrival_stop_id()) {
        Some(stop_connections) => stop_connections,
        None => return,
    }
    .into_iter()
    .filter(|stop_connection| {
        data_storage
            .stops()
            .data()
            .contains_key(&stop_connection.stop_id_2())
    })
    .filter(|stop_connection| !route.visited_stops().contains(&stop_connection.stop_id_2()))
    .map(|stop_connection| {
        clone_update_route(route, |cloned_sections, cloned_visited_stops| {
            cloned_sections.push(RouteSection::new(
                None,
                stop_connection.stop_id_1(),
                stop_connection.stop_id_2(),
                add_minutes_to_date_time(route.arrival_at(), stop_connection.duration().into()),
                Some(stop_connection.duration()),
            ));
            cloned_visited_stops.insert(stop_connection.stop_id_2());
        })
    })
    .for_each(|new_route| sorted_insert(routes, new_route));
}

fn filter_new_routes(new_routes: Vec<Route>, journeys_to_ignore: &HashSet<i32>) -> Vec<Route> {
    new_routes
        .into_iter()
        .filter(|route| !journeys_to_ignore.contains(&route.last_section().journey_id().unwrap()))
        .collect()
}
