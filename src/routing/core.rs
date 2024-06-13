use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;

use crate::storage::DataStorage;

use super::{
    connections::{
        create_initial_routes, create_route_from_another_route, get_connections,
        get_nearby_stop_connections,
    },
    constants::MAX_CONNECTION_COUNT,
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
    let mut solution: Option<Route> = None;
    let mut journeys_to_ignore = HashSet::new();
    let mut earliest_arrival_time_by_stop_id = HashMap::new();

    for i in 0..MAX_CONNECTION_COUNT {
        if verbose {
            println!("{}", routes.len());
        }

        let connections = process_routes(
            data_storage,
            routes,
            arrival_stop_id,
            i,
            &mut solution,
            &mut journeys_to_ignore,
            &mut earliest_arrival_time_by_stop_id,
        );

        if connections.is_empty() {
            break;
        } else {
            routes = connections;
        }
    }

    if verbose {
        if let Some(solution) = solution {
            println!("");
            solution.print(data_storage);
        }
    }
}

fn process_routes(
    data_storage: &DataStorage,
    mut routes: Vec<Route>,
    target_arrival_stop_id: i32,
    current_connection_count: i32,
    solution: &mut Option<Route>,
    journeys_to_ignore: &mut HashSet<i32>,
    earliest_arrival_time_by_stop_id: &mut HashMap<i32, NaiveDateTime>,
) -> Vec<Route> {
    let mut connections = Vec::new();

    while !routes.is_empty() {
        let route = routes.remove(0);

        if !can_improve_solution(&solution, &route) {
            continue;
        }

        if is_improving_solution(data_storage, &solution, &route, target_arrival_stop_id) {
            *solution = Some(route);
            continue;
        }

        let last_route_section = route.last_route_section();
        if let Some(journey_id) = last_route_section.journey_id() {
            journeys_to_ignore.insert(journey_id);

            create_route_from_another_route(
                data_storage,
                &route,
                journey_id,
                last_route_section.arrival_stop_id(),
                last_route_section.arrival_at(),
                target_arrival_stop_id,
            )
            .map(|r| sorted_insert(&mut routes, r));
        }

        if current_connection_count == MAX_CONNECTION_COUNT {
            continue;
        }

        if !can_explore_connections(&route, earliest_arrival_time_by_stop_id) {
            continue;
        }

        connections.extend(get_connections(
            data_storage,
            &route,
            target_arrival_stop_id,
        ));

        get_nearby_stop_connections(data_storage, &route)
            .into_iter()
            .for_each(|rou| sorted_insert(&mut routes, rou));
    }

    connections = filter_connections(connections, journeys_to_ignore);
    sort_routes(&mut connections);
    connections
}

fn can_improve_solution(solution: &Option<Route>, candidate: &Route) -> bool {
    if let Some(sol) = &solution {
        candidate.arrival_at() <= sol.arrival_at()
    } else {
        true
    }
}

fn is_improving_solution(
    data_storage: &DataStorage,
    solution: &Option<Route>,
    candidate: &Route,
    target_arrival_stop_id: i32,
) -> bool {
    fn count_stops(data_storage: &DataStorage, route_section: &RouteSection) -> i32 {
        route_section.journey(data_storage).unwrap().count_stops(
            route_section.departure_stop_id(),
            route_section.arrival_stop_id(),
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

    let route_sections_1 = candidate.route_sections_with_existing_journey();
    let route_sections_2 = solution.route_sections_with_existing_journey();

    for i in 0..connection_count_1 {
        let stop_count_1 = count_stops(data_storage, route_sections_1[i]);
        let stop_count_2 = count_stops(data_storage, route_sections_2[i]);

        if stop_count_1 != stop_count_2 {
            return stop_count_1 > stop_count_2;
        }
    }

    false
}

fn can_explore_connections(
    route: &Route,
    earliest_arrival_time_by_stop_id: &mut HashMap<i32, NaiveDateTime>,
) -> bool {
    let t1 = route.arrival_at();

    if earliest_arrival_time_by_stop_id.contains_key(&route.arrival_stop_id()) {
        let t2 = *earliest_arrival_time_by_stop_id
            .get(&route.arrival_stop_id())
            .unwrap();

        if t1 < t2 {
            *earliest_arrival_time_by_stop_id
                .get_mut(&route.arrival_stop_id())
                .unwrap() = t1;
        }

        t1 <= t2
    } else {
        earliest_arrival_time_by_stop_id.insert(route.arrival_stop_id(), t1);
        true
    }
}

fn filter_connections(connections: Vec<Route>, journeys_to_ignore: &HashSet<i32>) -> Vec<Route> {
    connections
        .into_iter()
        .filter(|connection| {
            !journeys_to_ignore.contains(&connection.last_route_section().journey_id().unwrap())
        })
        .collect()
}
