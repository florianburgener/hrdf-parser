mod connections;
mod constants;
mod core;
mod display;
mod exploration;
mod models;
mod utils;

use core::compute_routing;

use chrono::NaiveDateTime;
use models::{Route, RoutingAlgorithmArgs};

use crate::hrdf::Hrdf;

impl Hrdf {
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
}
