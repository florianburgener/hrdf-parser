use std::{cell::Ref, collections::HashSet};

use chrono::NaiveDate;

use crate::{
    hrdf::Hrdf,
    models::{Journey, Time},
};

impl Hrdf {
    pub fn plan_trip(
        &self,
        departure_stop_id: i32,
        arrival_stop_id: i32,
        departure_date: NaiveDate,
        _departure_time: Time,
    ) {
        let data_storage = self.data_storage();

        let _departure_stop = data_storage.stops().find(departure_stop_id);
        let _arrival_stop = data_storage.stops().find(arrival_stop_id);

        let journeys = self.get_operating_journeys(departure_date, departure_stop_id);
        for route_entry in journeys[0].route() {
            println!(
                "{} {:?} {:?}",
                route_entry.stop().name(),
                route_entry.arrival_time(),
                route_entry.departure_time()
            );
        }
    }

    fn get_operating_journeys(&self, date: NaiveDate, stop_id: i32) -> Vec<Ref<Journey>> {
        let data_storage = self.data_storage();

        let journeys_1 = data_storage.journeys().find_by_day(date);
        let journeys_2 = data_storage.journeys().find_by_stop_id(stop_id);

        let ids: HashSet<i32> = journeys_1.intersection(&journeys_2).cloned().collect();

        ids.into_iter()
            .map(|id| Ref::map(self.data_storage(), |d| d.journeys().find(id)))
            .collect()
    }
}
