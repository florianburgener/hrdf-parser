use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;

use crate::storage::DataStorage;

use super::{
    connections::create_initial_routes,
    constants::MAXIMUM_NUMBER_OF_EXPLORABLE_CONNECTIONS,
    exploration::explore_routes,
    models::{Route, RouteSection, RoutingAlgorithmArgs, RoutingAlgorithmMode},
};

pub fn compute_routing(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    departure_at: NaiveDateTime,
    verbose: bool,
    args: RoutingAlgorithmArgs,
) -> HashMap<i32, Route> {
    let mut routes = create_initial_routes(data_storage, departure_stop_id, departure_at);
    let mut journeys_to_ignore = HashSet::new();
    let mut earliest_arrival_by_stop_id = HashMap::new();
    let mut solutions = HashMap::new();

    for _ in 0..MAXIMUM_NUMBER_OF_EXPLORABLE_CONNECTIONS {
        if verbose {
            println!("{}", routes.len());
        }

        let can_continue_exploration = match args.mode() {
            RoutingAlgorithmMode::SolveFromDepartureStopToArrivalStop => |route: &_| {
                can_continue_exploration_one_to_one(
                    data_storage,
                    route,
                    &mut solutions,
                    args.arrival_stop_id(),
                )
            },
            // RoutingAlgorithmMode::SolveFromDepartureStopToReachableArrivalStops => panic!(),
        };

        let new_routes = explore_routes(
            data_storage,
            routes,
            &mut journeys_to_ignore,
            &mut earliest_arrival_by_stop_id,
            can_continue_exploration,
        );

        if new_routes.is_empty() {
            break;
        }

        routes = new_routes;
    }

    if verbose && args.mode() == RoutingAlgorithmMode::SolveFromDepartureStopToArrivalStop {
        if let Some(sol) = solutions.get(&args.arrival_stop_id()) {
            println!();
            sol.print(data_storage);
        }
    }

    solutions
}

fn can_continue_exploration_one_to_one(
    data_storage: &DataStorage,
    route: &Route,
    solutions: &mut HashMap<i32, Route>,
    arrival_stop_id: i32,
) -> bool {
    let solution = solutions.get(&arrival_stop_id);

    if !can_improve_solution(route, &solution) {
        return false;
    }

    if !route.visited_stops().contains(&arrival_stop_id) {
        return true;
    }

    let mut cloned_route = route.clone();
    cloned_route
        .last_section_mut()
        .set_arrival_stop_id(arrival_stop_id);

    if is_improving_solution(data_storage, &cloned_route, &solution) {
        solutions.insert(arrival_stop_id, cloned_route);
    }

    false
}

fn can_improve_solution(candidate: &Route, solution: &Option<&Route>) -> bool {
    solution
        .as_ref()
        .map_or(true, |sol| candidate.arrival_at() <= sol.arrival_at())
}

fn is_improving_solution(
    data_storage: &DataStorage,
    candidate: &Route,
    solution: &Option<&Route>,
) -> bool {
    fn count_stops(data_storage: &DataStorage, section: &RouteSection) -> i32 {
        section
            .journey(data_storage)
            .unwrap()
            .count_stops(section.departure_stop_id(), section.arrival_stop_id())
    }

    if solution.is_none() {
        return true;
    }

    let solution = solution.unwrap();

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
