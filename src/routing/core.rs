use chrono::NaiveDateTime;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{storage::DataStorage, utils::sub_1_day};

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
) -> FxHashMap<i32, Route> {
    let mut routes = create_initial_routes(data_storage, departure_stop_id, departure_at);
    let mut journeys_to_ignore = FxHashSet::default();
    let mut earliest_arrival_by_stop_id = FxHashMap::default();
    let mut solutions = FxHashMap::default();

    for _ in 0..MAXIMUM_NUMBER_OF_EXPLORABLE_CONNECTIONS {
        if verbose {
            println!("{}", routes.len());
        }

        let can_continue_exploration: Box<dyn FnMut(&Route) -> bool> = match args.mode() {
            RoutingAlgorithmMode::SolveFromDepartureStopToArrivalStop => Box::new(|route| {
                can_continue_exploration_one_to_one(
                    data_storage,
                    route,
                    &mut solutions,
                    args.arrival_stop_id(),
                )
            }),
            RoutingAlgorithmMode::SolveFromDepartureStopToReachableArrivalStops => {
                Box::new(|route| {
                    can_continue_exploration_one_to_many(
                        data_storage,
                        route,
                        &mut solutions,
                        args.time_limit(),
                    )
                })
            }
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
    solutions: &mut FxHashMap<i32, Route>,
    arrival_stop_id: i32,
) -> bool {
    let solution = solutions.get(&arrival_stop_id);

    if !route.visited_stops().contains(&arrival_stop_id) {
        return can_improve_solution(route, &solution);
    }

    let candidate = if route.last_section().journey_id().is_none() {
        route.clone()
    } else {
        update_arrival_stop(data_storage, route.clone(), arrival_stop_id)
    };

    if is_improving_solution(data_storage, &candidate, &solution) {
        solutions.insert(arrival_stop_id, candidate);
    }

    false
}

fn can_continue_exploration_one_to_many(
    data_storage: &DataStorage,
    route: &Route,
    solutions: &mut FxHashMap<i32, Route>,
    time_limit: NaiveDateTime,
) -> bool {
    fn evaluate_candidate(
        data_storage: &DataStorage,
        candidate: Route,
        solutions: &mut FxHashMap<i32, Route>,
        time_limit: NaiveDateTime,
    ) {
        if candidate.arrival_at() > time_limit {
            return;
        }

        let arrival_stop_id = candidate.last_section().arrival_stop_id();
        let solution = solutions.get(&arrival_stop_id);

        if is_improving_solution(data_storage, &candidate, &solution) {
            solutions.insert(arrival_stop_id, candidate);
        }
    }

    if route.last_section().journey_id().is_none() {
        evaluate_candidate(data_storage, route.clone(), solutions, time_limit);
    } else {
        let last_section = route.last_section();
        let journey = last_section.journey(data_storage).unwrap();

        for route_entry in journey.route_section(
            last_section.departure_stop_id(),
            last_section.arrival_stop_id(),
        ) {
            let candidate = update_arrival_stop(data_storage, route.clone(), route_entry.stop_id());
            evaluate_candidate(data_storage, candidate, solutions, time_limit);
        }
    }

    route.last_section().arrival_at() < time_limit
}

/// Do not call this function if route.last_section().journey_id() is None.
fn update_arrival_stop(
    data_storage: &DataStorage,
    mut route: Route,
    arrival_stop_id: i32,
) -> Route {
    let journey = route.last_section().journey(data_storage).unwrap();
    let new_arrival_time = journey.arrival_time_of(arrival_stop_id);

    let last_section = route.last_section_mut();

    let arrival_at = last_section.arrival_at();
    let arrival_date = arrival_at.date();

    let new_arrival_at = if new_arrival_time <= arrival_at.time() {
        NaiveDateTime::new(arrival_date, new_arrival_time)
    } else {
        NaiveDateTime::new(sub_1_day(arrival_date), new_arrival_time)
    };

    last_section.set_arrival_stop_id(arrival_stop_id);
    last_section.set_arrival_at(new_arrival_at);

    route
}

fn can_improve_solution(route: &Route, solution: &Option<&Route>) -> bool {
    solution
        .as_ref()
        .map_or(true, |sol| route.arrival_at() <= sol.arrival_at())
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
        // If this is the first solution found, then we keep the candidate as the solution.
        return true;
    }

    let solution = solution.unwrap();

    // A variable suffixed with 1 will always correspond to the candiate, suffixed with 2 will correspond to the solution.
    let t1 = candidate.arrival_at();
    let t2 = solution.arrival_at();

    if t1 != t2 {
        // If the candidate arrives earlier than the solution, then it is a better solution.
        return t1 < t2;
    }

    let connection_count_1 = candidate.count_connections();
    let connection_count_2 = solution.count_connections();

    if connection_count_1 != connection_count_2 {
        // If the candidate requires fewer connections, then it is a better solution.
        return connection_count_1 < connection_count_2;
    }

    let sections_1 = candidate.sections_having_journey();
    let sections_2 = solution.sections_having_journey();

    // Compare each connection.
    for i in 0..connection_count_1 {
        let stop_count_1 = count_stops(data_storage, sections_1[i]);
        let stop_count_2 = count_stops(data_storage, sections_2[i]);

        if stop_count_1 != stop_count_2 {
            // If the candidate crosses more stops than the solution, then it is a better solution.
            return stop_count_1 > stop_count_2;
        }
    }

    // The current solution is better than the candidate.
    false
}
