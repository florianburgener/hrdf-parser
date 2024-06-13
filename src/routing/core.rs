use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;

use crate::storage::DataStorage;

use super::{
    connections::{
        create_initial_routes, create_route_from_another_route, get_connections,
        get_connections_from_explorable_nearby_stops,
    },
    constants::MAXIMUM_NUMBER_OF_EXPLORABLE_CONNECTIONS,
    models::{Route, RouteSection},
    utils::{sort_routes, sorted_insert},
};

pub fn find_solution(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    arrival_stop_id: i32,
    departure_at: NaiveDateTime,
    verbose: bool,
) {
    let mut routes = create_initial_routes(
        data_storage,
        departure_stop_id,
        arrival_stop_id,
        departure_at,
    );
    let mut solution = None;
    let mut journeys_to_ignore = HashSet::new();
    let mut earliest_arrival_by_stop_id = HashMap::new();

    for num_explored_connections in 0..MAXIMUM_NUMBER_OF_EXPLORABLE_CONNECTIONS {
        if verbose {
            println!("{}", routes.len());
        }

        routes = process_routes(
            data_storage,
            routes,
            arrival_stop_id,
            num_explored_connections,
            &mut solution,
            &mut journeys_to_ignore,
            &mut earliest_arrival_by_stop_id,
        );

        if routes.is_empty() {
            break;
        }
    }

    if verbose {
        if let Some(solution) = solution {
            println!();
            solution.print(data_storage);
        }
    }
}

fn process_routes(
    data_storage: &DataStorage,
    mut routes: Vec<Route>,
    arrival_stop_id: i32,
    num_explored_connections: i32,
    solution: &mut Option<Route>,
    journeys_to_ignore: &mut HashSet<i32>,
    earliest_arrival_by_stop_id: &mut HashMap<i32, NaiveDateTime>,
) -> Vec<Route> {
    let mut next_routes = Vec::new();

    while !routes.is_empty() {
        let route = routes.remove(0);

        if !can_improve_solution(solution, &route) {
            continue;
        }

        if is_improving_solution(data_storage, solution, &route, arrival_stop_id) {
            *solution = Some(route);
            continue;
        }

        let last_section = route.last_section();
        if let Some(journey_id) = last_section.journey_id() {
            journeys_to_ignore.insert(journey_id);

            if let Some(new_route) = create_route_from_another_route(
                data_storage,
                &route,
                journey_id,
                last_section.arrival_stop_id(),
                last_section.arrival_at(),
                arrival_stop_id,
            ) {
                sorted_insert(&mut routes, new_route)
            }
        }

        if num_explored_connections == MAXIMUM_NUMBER_OF_EXPLORABLE_CONNECTIONS {
            continue;
        }

        if !can_explore_connections(&route, earliest_arrival_by_stop_id) {
            continue;
        }

        next_routes.extend(get_connections(data_storage, &route, arrival_stop_id));

        get_connections_from_explorable_nearby_stops(data_storage, &route)
            .into_iter()
            .for_each(|new_route| sorted_insert(&mut routes, new_route));
    }

    next_routes = filter_next_routes(next_routes, journeys_to_ignore);
    sort_routes(&mut next_routes);
    next_routes
}

fn can_improve_solution(solution: &Option<Route>, candidate: &Route) -> bool {
    solution
        .as_ref()
        .map_or(true, |sol| candidate.arrival_at() <= sol.arrival_at())
}

fn is_improving_solution(
    data_storage: &DataStorage,
    solution: &Option<Route>,
    candidate: &Route,
    target_arrival_stop_id: i32,
) -> bool {
    fn count_stops(data_storage: &DataStorage, section: &RouteSection) -> i32 {
        section.journey(data_storage).unwrap().count_stops(
            section.departure_stop_id(),
            section.arrival_stop_id(),
        )
    }

    if candidate.arrival_stop_id() != target_arrival_stop_id {
        return false;
    }

    if solution.is_none() {
        return true;
    }

    let solution = solution.as_ref().unwrap();

    let t1 = candidate.arrival_at();
    let t2 = solution.arrival_at();

    if t1 != t2 {
        return t1 < t2;
    }

    let connection_count_1 = candidate.count_connections();
    let connection_count_2 = solution.count_connections();

    if connection_count_1 != connection_count_2 {
        return connection_count_1 < connection_count_2;
    }

    let sections_1 = candidate.sections_having_journey();
    let sections_2 = solution.sections_having_journey();

    for i in 0..connection_count_1 {
        let stop_count_1 = count_stops(data_storage, sections_1[i]);
        let stop_count_2 = count_stops(data_storage, sections_2[i]);

        if stop_count_1 != stop_count_2 {
            return stop_count_1 > stop_count_2;
        }
    }

    false
}

fn can_explore_connections(
    route: &Route,
    earliest_arrival_by_stop_id: &mut HashMap<i32, NaiveDateTime>,
) -> bool {
    let arrival_at = route.arrival_at();
    let stop_id = route.arrival_stop_id();

    if let Some(&earliest_known_arrival) = earliest_arrival_by_stop_id.get(&stop_id) {
        // WARNING: Consider putting the "<=" back.
        if arrival_at < earliest_known_arrival {
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

fn filter_next_routes(next_routes: Vec<Route>, journeys_to_ignore: &HashSet<i32>) -> Vec<Route> {
    next_routes
        .into_iter()
        .filter(|route| !journeys_to_ignore.contains(&route.last_section().journey_id().unwrap()))
        .collect()
}
