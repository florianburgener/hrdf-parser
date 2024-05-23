use crate::{hrdf::Hrdf, models::Time};

impl Hrdf {
    pub fn plan_trip(&self, departure_stop_id: i32, arrival_stop_id: i32, departure_time: Time) {
        let departure_stop = self
            .stops()
            .primary_index()
            .get(&departure_stop_id)
            .unwrap();

        let arrival_stop = self
            .stops()
            .primary_index()
            .get(&arrival_stop_id)
            .unwrap();

        println!("{} {}", departure_stop.name(), arrival_stop.name());
    }
}
