use std::time::Instant;

use chrono::NaiveDate;

use crate::{
    hrdf::Hrdf,
    models::{Journey, ResourceCollection, Time},
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
        for _ in 0..100 {
            let journeys = self.get_operating_journeys(departure_date, departure_stop_id);
            x += journeys.len();
        }
        let elapsed = now.elapsed() / 100;
        println!("Elapsed: {:.2?}", elapsed);

        println!("{}", x);
    }

    fn get_operating_journeys(&self, date: NaiveDate, stop_id: i32) -> ResourceCollection<Journey> {
        let data_storage = self.data_storage();
        let journeys = data_storage.journeys();

        let journeys_by_day = journeys.journeys_by_day();
        let journeys_1 = journeys_by_day.get(&date).unwrap();
        let journeys_2 = journeys.find_journeys_for_specific_stop_id(stop_id);

        let ids = journeys_1.intersection(&journeys_2).cloned().collect();

        data_storage.journeys().get(ids)
    }
}
