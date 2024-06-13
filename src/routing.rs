mod connections;
mod constants;
mod core;
mod display;
mod models;
mod utils;

use core::find_solution;

use chrono::NaiveDateTime;

use crate::hrdf::Hrdf;

impl Hrdf {
    pub fn plan_journey(
        &self,
        departure_stop_id: i32,
        arrival_stop_id: i32,
        departure_at: NaiveDateTime,
        verbose: bool,
    ) {
        find_solution(
            self.data_storage(),
            departure_stop_id,
            arrival_stop_id,
            departure_at,
            verbose,
        );
    }
}
