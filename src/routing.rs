mod connections;
mod constants;
mod core;
mod display;
mod exploration;
mod models;
mod route_impl;
mod utils;

pub use models::RouteResult;

use core::compute_routing;

use chrono::{Duration, NaiveDateTime};
use models::RoutingAlgorithmArgs;
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
    ) -> Option<RouteResult> {
        let result = compute_routing(
            self.data_storage(),
            departure_stop_id,
            departure_at,
            verbose,
            RoutingAlgorithmArgs::solve_from_departure_stop_to_arrival_stop(arrival_stop_id),
        )
        .remove(&arrival_stop_id);

        if verbose {
            if let Some(rou) = &result {
                println!();
                rou.print(self.data_storage());
            }
        }

        result
    }

    #[allow(dead_code)]
    pub fn find_reachable_stops_within_time_limit(
        &self,
        departure_stop_id: i32,
        departure_at: NaiveDateTime,
        time_limit: Duration,
        verbose: bool,
    ) -> FxHashMap<i32, RouteResult> {
        compute_routing(
            self.data_storage(),
            departure_stop_id,
            departure_at,
            verbose,
            RoutingAlgorithmArgs::solve_from_departure_stop_to_reachable_arrival_stops(
                departure_at.checked_add_signed(time_limit).unwrap(),
            ),
        )
    }
}
