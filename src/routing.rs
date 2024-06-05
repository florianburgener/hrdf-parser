use std::collections::HashSet;

use chrono::NaiveDate;

use crate::{
    hrdf::Hrdf,
    models::{Journey, Model, Time},
    storage::DataStorage,
};

#[derive(Debug, Clone)]
struct RouteSection {
    journey_id: i32,
    departure_stop_id: i32,
    arrival_stop_id: i32,
}

impl RouteSection {
    pub fn new(journey_id: i32, departure_stop_id: i32, arrival_stop_id: i32) -> Self {
        Self {
            journey_id,
            departure_stop_id,
            arrival_stop_id,
        }
    }

    // Getters/Setters

    pub fn journey_id(&self) -> i32 {
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

    // Functions

    pub fn journey<'a>(&'a self, data_storage: &'a DataStorage) -> &Journey {
        data_storage.journeys().find(self.journey_id())
    }
}

#[derive(Debug, Clone)]
struct Node {
    route_sections: Vec<RouteSection>,
    visited_stops: HashSet<i32>,
}

impl Node {
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

    pub fn arrival_stop_id(&self) -> i32 {
        self.route_sections().last().unwrap().arrival_stop_id()
    }

    pub fn arrival_time<'a>(&'a self, data_storage: &'a DataStorage) -> &Time {
        let route_section = self.route_sections().last().unwrap();

        route_section
            .journey(data_storage)
            .route()
            .iter()
            .skip(1)
            .find(|route_entry| route_entry.stop_id() == route_section.arrival_stop_id())
            .unwrap()
            .arrival_time()
            .as_ref()
            // TODO: it could crash here.
            .unwrap()
    }

    pub fn last_route_section(&self) -> &RouteSection {
        self.route_sections().last().unwrap()
    }

    pub fn print(&self, data_storage: &DataStorage) {
        for route_section in self.route_sections() {
            let journey = route_section.journey(data_storage);
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

const MAX_CONNECTION_COUNT: i32 = 2;

fn find_solution(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    arrival_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: Time,
    verbose: bool,
) {
    let mut connections = create_initial_connections(
        data_storage,
        departure_stop_id,
        arrival_stop_id,
        departure_date,
        &departure_time,
    );
    let mut solution: Option<Node> = None;
    let mut aaa = HashSet::new();
    let mut current_connection_count = 0;

    while !connections.is_empty() {
        if verbose {
            println!("{}", connections.len());
        }

        connections = process_connections(
            data_storage,
            connections,
            &mut solution,
            &mut aaa,
            arrival_stop_id,
            current_connection_count,
            departure_date,
        );
        current_connection_count += 1;
    }

    if verbose {
        if let Some(s) = solution {
            // println!("{:#?}", best_solution);
            s.print(data_storage);
        }
    }
}

fn create_initial_connections(
    data_storage: &DataStorage,
    departure_stop_id: i32,
    target_arrival_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: &Time,
) -> Vec<Node> {
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
            Node::new(vec![route_section], visited_stops)
        })
    })
    .collect();
    sort_connections(data_storage, &mut connections);
    connections
}

fn next_departures<'a>(
    data_storage: &'a DataStorage,
    stop_id: i32,
    date: NaiveDate,
    departure_time: &'a Time,
    routes_to_ignore: Option<HashSet<u64>>,
) -> Vec<&'a Journey> {
    let mut journeys: Vec<&Journey> = get_operating_journeys(data_storage, date, stop_id);

    let departure_time_max = *departure_time + Time::new(1, 0);
    journeys = journeys
        .into_iter()
        .filter(|journey| !journey.is_last_stop(stop_id))
        .filter(|journey| {
            let journey_departure_time = get_departure_time(journey, stop_id);
            journey_departure_time >= departure_time
                && journey_departure_time <= &departure_time_max
        })
        .collect();

    journeys.sort_by(|a, b| {
        let a = get_departure_time(a, stop_id);
        let b = get_departure_time(b, stop_id);
        a.cmp(b)
    });

    let mut unique_route = HashSet::new();

    if let Some(routes_to_ignore) = routes_to_ignore {
        unique_route = routes_to_ignore;
    }

    journeys
        .into_iter()
        .filter_map(|journey| {
            let hash = journey.hash_route(stop_id).unwrap();

            if unique_route.contains(&hash) {
                None
            } else {
                unique_route.insert(hash);
                Some(journey)
            }
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

    let ids: HashSet<i32> = journeys_1.intersection(&journeys_2).cloned().collect();

    data_storage.journeys().resolve_ids(data_storage, ids)
}

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
                RouteSection::new(journey.id(), departure_stop_id, stop.id()),
                visited_stops,
            ));
        }

        route_iter.next();
    }

    None
}

fn sort_connections(data_storage: &DataStorage, connections: &mut Vec<Node>) {
    connections.sort_by(|a, b| {
        a.arrival_time(data_storage)
            .cmp(b.arrival_time(data_storage))
    });
}

fn process_connections(
    data_storage: &DataStorage,
    mut connections: Vec<Node>,
    solution: &mut Option<Node>,
    aaa: &mut HashSet<i32>,
    target_arrival_stop_id: i32,
    current_connection_count: i32,
    departure_date: NaiveDate,
) -> Vec<Node> {
    let mut next_connections = Vec::new();

    while !connections.is_empty() {
        let connection = connections.remove(0);

        if is_improving_solution(data_storage, &solution, &connection, target_arrival_stop_id) {
            *solution = Some(connection);
            continue;
        }

        if !can_improve_solution(data_storage, &solution, &connection) {
            continue;
        }

        let last_route_section = connection.last_route_section();
        aaa.insert(last_route_section.journey_id());

        next_connection(
            data_storage,
            &connection,
            last_route_section.journey_id(),
            last_route_section.arrival_stop_id(),
            target_arrival_stop_id,
        )
        .map(|c| sorted_insert(data_storage, &mut connections, c));

        if current_connection_count == MAX_CONNECTION_COUNT {
            continue;
        }

        let routes_to_ignore = connection
            .route_sections()
            .iter()
            .filter_map(|route_section| {
                route_section
                    .journey(data_storage)
                    .hash_route(connection.arrival_stop_id())
            })
            .collect();

        let next_connections_to_insert: Vec<Node> = next_departures(
            data_storage,
            connection.arrival_stop_id(),
            departure_date,
            connection.arrival_time(data_storage),
            Some(routes_to_ignore),
        )
        .iter()
        .filter_map(|j| {
            next_connection(
                data_storage,
                &connection,
                j.id(),
                last_route_section.arrival_stop_id(),
                target_arrival_stop_id,
            )
        })
        .collect();
        next_connections.extend(next_connections_to_insert);
    }

    next_connections = next_connections
        .into_iter()
        .filter(|c| !aaa.contains(&c.last_route_section().journey_id()))
        .collect();

    sort_connections(data_storage, &mut next_connections);
    next_connections
}

fn is_improving_solution(
    data_storage: &DataStorage,
    solution: &Option<Node>,
    candidate: &Node,
    target_arrival_stop_id: i32,
) -> bool {
    fn count_stops(data_storage: &DataStorage, node: &Node, i: usize) -> i32 {
        let route_section = &node.route_sections()[i];

        route_section.journey(data_storage).count_stops(
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

    let connection_count_1 = candidate.route_sections().len();
    let connection_count_2 = solution.route_sections().len();

    if connection_count_1 != connection_count_2 {
        return connection_count_1 < connection_count_2;
    }

    for i in 0..connection_count_1 {
        let stop_count_1 = count_stops(data_storage, candidate, i);
        let stop_count_2 = count_stops(data_storage, solution, i);

        if stop_count_1 != stop_count_2 {
            return stop_count_1 > stop_count_2;
        }
    }

    false
}

fn can_improve_solution(
    data_storage: &DataStorage,
    solution: &Option<Node>,
    candidate: &Node,
) -> bool {
    if let Some(s) = &solution {
        candidate.arrival_time(data_storage) <= s.arrival_time(data_storage)
    } else {
        true
    }
}

fn next_connection(
    data_storage: &DataStorage,
    connection: &Node,
    journey_id: i32,
    departure_stop_id: i32,
    target_arrival_stop_id: i32,
) -> Option<Node> {
    let is_same_journey = connection.last_route_section().journey_id() == journey_id;

    get_next_route_section(
        data_storage,
        data_storage.journeys().find(journey_id),
        departure_stop_id,
        target_arrival_stop_id,
        is_same_journey,
    )
    .and_then(|(new_route_section, new_visited_stops)| {
        if has_intersecting_stops(connection.visited_stops(), &new_visited_stops) {
            return None;
        }

        let mut cloned_route_sections: Vec<RouteSection> = connection.route_sections().clone();
        let mut cloned_visited_stops = connection.visited_stops().clone();

        if is_same_journey {
            cloned_route_sections
                .last_mut()
                .unwrap()
                .set_arrival_stop_id(new_route_section.arrival_stop_id());
        } else {
            cloned_route_sections.push(new_route_section);
        }

        cloned_visited_stops.extend(new_visited_stops);

        Some(Node::new(cloned_route_sections, cloned_visited_stops))
    })
}

fn has_intersecting_stops(
    visited_stops: &HashSet<i32>,
    visited_stops_other: &HashSet<i32>,
) -> bool {
    visited_stops.intersection(visited_stops_other).count() != 0
}

fn sorted_insert(
    data_storage: &DataStorage,
    connections: &mut Vec<Node>,
    connection_to_insert: Node,
) {
    let mut i = 0;

    while i < connections.len() {
        let t1 = connection_to_insert.arrival_time(data_storage);
        let t2 = connections[i].arrival_time(data_storage);

        if t1 < t2 {
            connections.insert(i, connection_to_insert);
            return;
        }

        i += 1;
    }

    connections.push(connection_to_insert);
}
