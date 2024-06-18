use chrono::NaiveDate;
use rustc_hash::FxHashSet;

use crate::{
    models::{Journey, StopConnection},
    storage::DataStorage,
};

use super::models::{Route, RouteSection};

pub fn clone_update_route<F>(route: &Route, f: F) -> Route
where
    F: FnOnce(&mut Vec<RouteSection>, &mut FxHashSet<i32>),
{
    let mut cloned_sections = route.sections().clone();
    let mut cloned_visited_stops = route.visited_stops().clone();

    f(&mut cloned_sections, &mut cloned_visited_stops);

    Route::new(cloned_sections, cloned_visited_stops)
}

pub fn get_stop_connections(
    data_storage: &DataStorage,
    stop_id: i32,
) -> Option<Vec<&StopConnection>> {
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

    data_storage
        .journeys()
        .find_by_stop_id(stop_id)
        .map_or(Vec::new(), |journeys_2| {
            let ids = journeys_1.intersection(&journeys_2).cloned().collect();
            data_storage.journeys().resolve_ids(&ids)
        })
}

pub fn get_routes_to_ignore(data_storage: &DataStorage, route: &Route) -> FxHashSet<u64> {
    route
        .sections()
        .iter()
        .filter_map(|section| {
            section
                .journey(data_storage)
                .and_then(|journey| journey.hash_route(route.arrival_stop_id()))
        })
        .collect()
}

pub fn sort_routes(routes: &mut Vec<Route>) {
    routes.sort_by_key(|route| route.arrival_at());
}

pub fn sorted_insert(routes: &mut Vec<Route>, route_to_insert: Route) {
    let index = routes
        .iter()
        .position(|route| route_to_insert.arrival_at() < route.arrival_at())
        .unwrap_or_else(|| routes.len());
    routes.insert(index, route_to_insert);
}
