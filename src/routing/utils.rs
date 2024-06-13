use std::collections::HashSet;

use chrono::NaiveDate;

use crate::{
    models::{Journey, StopConnection},
    storage::DataStorage,
};

use super::models::Route;

pub fn get_nearby_stops(data_storage: &DataStorage, stop_id: i32) -> Option<Vec<&StopConnection>> {
    data_storage
        .stop_connections()
        .find_by_stop_id(stop_id)
        .map(|ids| data_storage.stop_connections().resolve_ids(ids))
}

pub fn get_operating_journeys(
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

pub fn get_routes_to_ignore(data_storage: &DataStorage, route: &Route) -> HashSet<u64> {
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

pub fn sort_routes(routes: &mut Vec<Route>) {
    routes.sort_by(|a, b| a.arrival_at().cmp(&b.arrival_at()));
}

pub fn sorted_insert(routes: &mut Vec<Route>, route_to_insert: Route) {
    let mut i = 0;

    while i < routes.len() {
        let t1 = route_to_insert.arrival_at();
        let t2 = routes[i].arrival_at();

        if t1 < t2 {
            routes.insert(i, route_to_insert);
            return;
        }

        i += 1;
    }

    routes.push(route_to_insert);
}
