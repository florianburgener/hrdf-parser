mod connections;
mod constants;
mod core;
mod display;
mod exploration;
mod models;
mod route;
mod utils;

use core::compute_routing;

use chrono::{Duration, NaiveDateTime};
use models::{Route, RoutingAlgorithmArgs};
use rustc_hash::FxHashMap;

use crate::hrdf::Hrdf;

impl Hrdf {
    #[allow(dead_code)]
    pub fn plan_journey(
        &self,
        departure_stop_id: i32,
        arrival_stop_id: i32,
        departure_at: NaiveDateTime,
        verbose: bool,
    ) -> Option<Route> {
        compute_routing(
            self.data_storage(),
            departure_stop_id,
            departure_at,
            verbose,
            RoutingAlgorithmArgs::solve_from_departure_stop_to_arrival_stop(arrival_stop_id),
        )
        .remove(&arrival_stop_id)
    }

    #[allow(dead_code)]
    pub fn find_reachable_stops_within_time_limit(
        &self,
        departure_stop_id: i32,
        departure_at: NaiveDateTime,
        time_limit: Duration,
        verbose: bool,
    ) -> FxHashMap<i32, Route> {
        compute_routing(
            self.data_storage(),
            departure_stop_id,
            departure_at,
            verbose,
            RoutingAlgorithmArgs::create_solve_one_to_many(
                departure_at.checked_add_signed(time_limit).unwrap(),
            ),
        )
    }
}
