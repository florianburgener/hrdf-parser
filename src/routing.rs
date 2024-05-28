use std::{cell::Ref, collections::HashSet, time::Instant};

use chrono::NaiveDate;

use crate::{
    hrdf::Hrdf,
    models::{Journey, Time}, resolve_ids,
};

impl Hrdf {
    pub fn plan_trip(
        &self,
        departure_stop_id: i32,
        _arrival_stop_id: i32,
        departure_date: NaiveDate,
        _departure_time: Time,
    ) {
        // let data_storage = self.data_storage();

        // let departure_stop = data_storage.stops().find(departure_stop_id);
        // let arrival_stop = data_storage.stops().find(arrival_stop_id);

        let now = Instant::now();
        let mut x = 0;
        for _ in 0..1000 {
            let journeys = self.get_operating_journeys(departure_date, departure_stop_id);
            x += journeys.len();
        }
        let elapsed = now.elapsed() / 1000;
        println!("Elapsed: {:.2?}", elapsed);

        println!("{}", x);
    }

    fn get_operating_journeys(&self, date: NaiveDate, stop_id: i32) -> Vec<Ref<Journey>> {
        let data_storage = self.data_storage();

        let journeys_1 = data_storage.journeys().find_by_day(date);
        let journeys_2 = data_storage.journeys().find_by_stop_id(stop_id);

        let ids: HashSet<i32> = journeys_1.intersection(&journeys_2).cloned().collect();
        resolve_ids!(self, ids, journeys)
    }
}
