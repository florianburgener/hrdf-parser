use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;

use crate::{
    hrdf::Hrdf,
    models::{Journey, Model, Time},
    storage::DataStorage,
};

const MAX_CONNECTION_COUNT: i32 = 6;

#[derive(Debug, Clone)]
struct RouteSection {
    journey_id: Option<i32>,
    departure_stop_id: i32,
    arrival_stop_id: i32,
    duration: Option<i16>,
}

impl RouteSection {
    pub fn new(
        journey_id: Option<i32>,
        departure_stop_id: i32,
        arrival_stop_id: i32,
        duration: Option<i16>,
    ) -> Self {
        Self {
            journey_id,
            departure_stop_id,
            arrival_stop_id,
            duration,
        }
    }

    // Getters/Setters

    pub fn journey_id(&self) -> Option<i32> {
        self.journey_id
    }

    pub fn departure_stop_id(&self) -> i32 {
        self.departure_stop_id
    }

    pub fn arrival_stop_id(&self) -> i32 {
        self.arrival_stop_id
    }

    pub fn set_arrival_stop_id(&mut self, value: i32) {
        self.arrival_stop_id = value;
    }

    pub fn duration(&self) -> &Option<i16> {
        &self.duration
    }

    // Functions

    pub fn journey<'a>(&'a self, data_storage: &'a DataStorage) -> Option<&Journey> {
        self.journey_id()
            .map(|journey_id| data_storage.journeys().find(journey_id))
    }

    pub fn arrival_time(&self, data_storage: &DataStorage) -> Option<Time> {
        self.journey(data_storage).map(|journey| {
            journey
                .route()
                .iter()
                .skip(1)
                .find(|route_entry| route_entry.stop_id() == self.arrival_stop_id())
                .unwrap()
                .arrival_time()
                .as_ref()
                .unwrap()
                .clone()
        })
    }
}

#[derive(Debug)]
struct Route {
    route_sections: Vec<RouteSection>,
    visited_stops: HashSet<i32>,
}

impl Route {
    pub fn new(route_sections: Vec<RouteSection>, visited_stops: HashSet<i32>) -> Self {
        Self {
            route_sections,
            visited_stops,
        }
    }

    pub fn route_sections(&self) -> &Vec<RouteSection> {
        &self.route_sections
    }

    pub fn visited_stops(&self) -> &HashSet<i32> {
        &self.visited_stops
    }

    // Functions

    pub fn last_route_section(&self) -> &RouteSection {
        self.route_sections().last().unwrap()
    }

    pub fn arrival_stop_id(&self) -> i32 {
        self.last_route_section().arrival_stop_id()
    }

    pub fn arrival_time<'a>(&'a self, data_storage: &'a DataStorage) -> Time {
        self.last_route_section()
            .arrival_time(data_storage)
            .unwrap_or_else(|| {
                // WARNING: If the journey starts with a connection, there will be an error.
                // The solution is to store the global departure time in the route.
                let (i, route_section) = self
                    .route_sections()
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, route_section)| route_section.journey_id().is_some())
                    .unwrap();

                let arrival_time = route_section.arrival_time(data_storage).unwrap();
                let duration: i32 = self.route_sections()[i + 1..self.route_sections().len()]
                    .iter()
                    .map(|route_section| route_section.duration().unwrap() as i32)
                    .sum();

                arrival_time + Time::from(duration)
            })
    }

    pub fn has_visited_any_stops(&self, stops: &HashSet<i32>) -> bool {
        self.visited_stops().intersection(stops).count() != 0
    }

    pub fn route_sections_with_existing_journey(&self) -> Vec<&RouteSection> {
        self.route_sections()
            .iter()
            .filter(|route_section| route_section.journey_id().is_some())
            .collect()
    }

    pub fn count_connections(&self) -> usize {
        self.route_sections_with_existing_journey().len()
    }

    pub fn print(&self, data_storage: &DataStorage) {
        for route_section in self.route_sections() {
            let journey = route_section.journey(data_storage);

            if journey.is_none() {
                let stop = data_storage.stops().find(route_section.arrival_stop_id());
                println!(
                    "Approx. {}-minute walk to {}",
                    route_section.duration().unwrap(),
                    stop.name()
                );
                continue;
            }

            let journey = journey.unwrap();
            println!("Journey #{}", journey.id());

            let mut route_iter = journey.route().into_iter().peekable();

            while route_iter.peek().unwrap().stop_id() != route_section.departure_stop_id() {
                route_iter.next();
            }

            let mut route = Vec::new();

            loop {
                route.push(route_iter.next().unwrap());

                if route.last().unwrap().stop_id() == route_section.arrival_stop_id() {
                    break;
                }
            }

            for (i, route_entry) in route.iter().enumerate() {
                let arrival_time = if i == 0 {
                    " ".repeat(5)
                } else {
                    format!("{}", route_entry.arrival_time().as_ref().unwrap())
                };

                let departure_time = if i == route.len() - 1 {
                    " ".repeat(5)
                } else {
                    format!("{}", route_entry.departure_time().as_ref().unwrap())
                };

                let stop = route_entry.stop(data_storage);

                println!(
                    "  {} {: <36} {} - {} ; {}",
                    stop.id(),
                    stop.name(),
                    arrival_time,
                    departure_time,
                    stop.transfer_flag(),
                );
            }
        }
    }
}

impl Hrdf {
    pub fn plan_journey(
        &self,
        departure_stop_id: i32,
        arrival_stop_id: i32,
        departure_date: NaiveDate,
        departure_time: Time,
        verbose: bool,
    ) {
        find_solution(
            self.data_storage(),
            departure_stop_id,
            arrival_stop_id,
            departure_date,
            departure_time,
            verbose,
        );
    }
}

fn find_solution(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    arrival_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: Time,
    verbose: bool,
) {
    let mut routes = create_initial_routes(
        data_storage,
        departure_stop_id,
        arrival_stop_id,
        departure_date,
        departure_time,
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
            departure_date,
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
        if let Some(s) = solution {
            s.print(data_storage);
        }
    }
}

fn process_routes(
    data_storage: &DataStorage,
    mut routes: Vec<Route>,
    target_arrival_stop_id: i32,
    departure_date: NaiveDate,
    current_connection_count: i32,
    solution: &mut Option<Route>,
    journeys_to_ignore: &mut HashSet<i32>,
    earliest_arrival_time_by_stop_id: &mut HashMap<i32, Time>,
) -> Vec<Route> {
    let mut connections = Vec::new();

    while !routes.is_empty() {
        let route = routes.remove(0);

        if !can_improve_solution(data_storage, &solution, &route) {
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
                target_arrival_stop_id,
            )
            .map(|r| sorted_insert(data_storage, &mut routes, r));
        }

        if current_connection_count == MAX_CONNECTION_COUNT {
            continue;
        }

        if !can_explore_connections(data_storage, &route, earliest_arrival_time_by_stop_id) {
            continue;
        }

        connections.extend(get_connections(
            data_storage,
            &route,
            departure_date,
            target_arrival_stop_id,
        ));

        for r in explore_nearby_stops(data_storage, &route) {
            sorted_insert(data_storage, &mut routes, r);
        }
    }

    connections = filter_connections(connections, journeys_to_ignore);
    sort_routes(data_storage, &mut connections);
    connections
}

fn create_initial_routes(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    target_arrival_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: Time,
) -> Vec<Route> {
    let mut connections = next_departures(
        data_storage,
        departure_stop_id,
        departure_date,
        departure_time,
        None,
    )
    .iter()
    .filter_map(|journey| {
        get_next_route_section(
            data_storage,
            journey,
            departure_stop_id,
            target_arrival_stop_id,
            false,
        )
        .map(|(route_section, mut visited_stops)| {
            visited_stops.insert(departure_stop_id);
            Route::new(vec![route_section], visited_stops)
        })
    })
    .collect();
    sort_routes(data_storage, &mut connections);
    connections
}

fn next_departures<'a>(
    data_storage: &'a DataStorage,
    departure_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: Time,
    routes_to_ignore: Option<HashSet<u64>>,
) -> Vec<&'a Journey> {
    let mut journeys: Vec<&Journey> =
        get_operating_journeys(data_storage, departure_date, departure_stop_id);

    let departure_time_max = departure_time + Time::new(2, 0);
    journeys = journeys
        .into_iter()
        .filter(|journey| !journey.is_last_stop(departure_stop_id))
        .filter(|journey| {
            let journey_departure_time = get_departure_time(journey, departure_stop_id);
            *journey_departure_time >= departure_time
                && journey_departure_time <= &departure_time_max
        })
        .collect();

    journeys.sort_by(|a, b| {
        let a = get_departure_time(a, departure_stop_id);
        let b = get_departure_time(b, departure_stop_id);
        a.cmp(b)
    });

    let mut routes_to_ignore = routes_to_ignore.unwrap_or_else(HashSet::new);

    journeys
        .into_iter()
        .filter(|journey| {
            let hash = journey.hash_route(departure_stop_id).unwrap();
            let contains = routes_to_ignore.contains(&hash);

            if !contains {
                routes_to_ignore.insert(hash);
            }

            !contains
        })
        .collect()
}

fn get_operating_journeys(
    data_storage: &DataStorage,
    date: NaiveDate,
    stop_id: i32,
) -> Vec<&Journey> {
    let journeys_1 = data_storage.journeys().find_by_day(date);
    let journeys_2 = data_storage.journeys().find_by_stop_id(stop_id);

    journeys_2.map_or(Vec::new(), |journeys_2| {
        let ids: HashSet<i32> = journeys_1.intersection(&journeys_2).cloned().collect();
        data_storage.journeys().resolve_ids(&ids)
    })
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

    let t1 = candidate.arrival_time(data_storage);
    let t2 = solution.arrival_time(data_storage);

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

fn can_improve_solution(
    data_storage: &DataStorage,
    solution: &Option<Route>,
    candidate: &Route,
) -> bool {
    if let Some(s) = &solution {
        candidate.arrival_time(data_storage) <= s.arrival_time(data_storage)
    } else {
        true
    }
}

fn get_connections(
    data_storage: &DataStorage,
    route: &Route,
    departure_date: NaiveDate,
    target_arrival_stop_id: i32,
) -> Vec<Route> {
    next_departures(
        data_storage,
        route.arrival_stop_id(),
        departure_date,
        route.arrival_time(data_storage),
        Some(get_routes_to_ignore(data_storage, &route)),
    )
    .iter()
    .filter_map(|j| {
        create_route_from_another_route(
            data_storage,
            &route,
            j.id(),
            route.last_route_section().arrival_stop_id(),
            target_arrival_stop_id,
        )
    })
    .collect()
}

fn explore_nearby_stops(data_storage: &DataStorage, route: &Route) -> Vec<Route> {
    let ids = data_storage
        .stop_connections()
        .find_by_stop_id(route.arrival_stop_id());

    if ids.is_none() {
        return Vec::new();
    }

    let stop_connections = data_storage.stop_connections().resolve_ids(ids.unwrap());

    stop_connections
        .into_iter()
        .filter(|stop_connection| !route.visited_stops().contains(&stop_connection.stop_id_2()))
        .map(|stop_connection| {
            let mut cloned_route_sections = route.route_sections().clone();
            let mut cloned_visited_stops = route.visited_stops().clone();

            cloned_route_sections.push(RouteSection::new(
                None,
                stop_connection.stop_id_1(),
                stop_connection.stop_id_2(),
                Some(stop_connection.duration()),
            ));
            cloned_visited_stops.insert(stop_connection.stop_id_2());

            Route::new(cloned_route_sections, cloned_visited_stops)
        })
        .collect()
}

fn create_route_from_another_route(
    data_storage: &DataStorage,
    route: &Route,
    journey_id: i32,
    departure_stop_id: i32,
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
            cloned_route_sections
                .last_mut()
                .unwrap()
                .set_arrival_stop_id(new_route_section.arrival_stop_id());
        } else {
            cloned_route_sections.push(new_route_section);
        }

        cloned_visited_stops.extend(new_visited_stops);

        Some(Route::new(cloned_route_sections, cloned_visited_stops))
    })
}

fn get_next_route_section(
    data_storage: &DataStorage,
    journey: &Journey,
    departure_stop_id: i32,
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
            return Some((
                RouteSection::new(Some(journey.id()), departure_stop_id, stop.id(), None),
                visited_stops,
            ));
        }

        route_iter.next();
    }

    None
}

fn can_explore_connections(
    data_storage: &DataStorage,
    route: &Route,
    earliest_arrival_time_by_stop_id: &mut HashMap<i32, Time>,
) -> bool {
    let t1 = route.arrival_time(data_storage);

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

// ------------------------------------------------------------------------------------------------
// --- Helpers
// ------------------------------------------------------------------------------------------------

fn get_departure_time(journey: &Journey, stop_id: i32) -> &Time {
    journey
        .route()
        .iter()
        .find(|route_entry| route_entry.stop_id() == stop_id)
        .unwrap()
        .departure_time()
        .as_ref()
        .unwrap()
}

fn get_routes_to_ignore(data_storage: &DataStorage, route: &Route) -> HashSet<u64> {
    route
        .route_sections()
        .iter()
        .filter_map(|route_section| {
            route_section
                .journey(data_storage)
                .and_then(|journey| journey.hash_route(route.arrival_stop_id()))
        })
        .collect()
}

fn sort_routes(data_storage: &DataStorage, routes: &mut Vec<Route>) {
    routes.sort_by(|a, b| {
        a.arrival_time(data_storage)
            .cmp(&b.arrival_time(data_storage))
    });
}

fn sorted_insert(data_storage: &DataStorage, routes: &mut Vec<Route>, route_to_insert: Route) {
    let mut i = 0;

    while i < routes.len() {
        let t1 = route_to_insert.arrival_time(data_storage);
        let t2 = routes[i].arrival_time(data_storage);

        if t1 < t2 {
            routes.insert(i, route_to_insert);
            return;
        }

        i += 1;
    }

    routes.push(route_to_insert);
}
