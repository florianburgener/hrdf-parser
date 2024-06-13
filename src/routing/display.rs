use crate::{models::Model, storage::DataStorage};

use super::models::Route;

impl Route {
    pub fn print(&self, data_storage: &DataStorage) {
        for route_section in self.route_sections() {
            let journey = route_section.journey(data_storage);

            if journey.is_none() {
                let stop = data_storage.stops().find(route_section.arrival_stop_id());
                println!(
                    "Approx. {}-minute walk to {}",
                    route_section.duration().unwrap(),
                    stop.name()
                );
                continue;
            }

            let journey = journey.unwrap();
            println!("Journey #{}", journey.id());

            let mut route_iter = journey.route().into_iter().peekable();

            while route_iter.peek().unwrap().stop_id() != route_section.departure_stop_id() {
                route_iter.next();
            }

            let mut route = Vec::new();

            loop {
                route.push(route_iter.next().unwrap());

                if route.last().unwrap().stop_id() == route_section.arrival_stop_id() {
                    break;
                }
            }

            for (i, route_entry) in route.iter().enumerate() {
                let arrival_time = if i == 0 {
                    " ".repeat(5)
                } else {
                    format!("{}", route_entry.arrival_time().as_ref().unwrap())
                };

                let departure_time = if i == route.len() - 1 {
                    " ".repeat(5)
                } else {
                    format!("{}", route_entry.departure_time().as_ref().unwrap())
                };

                let stop = route_entry.stop(data_storage);

                println!(
                    "  {:0>7} {: <36} {} - {} ; {}",
                    stop.id(),
                    stop.name(),
                    arrival_time,
                    departure_time,
                    stop.transfer_flag(),
                );
            }

            println!("  Arrival date: {}", route_section.arrival_at().date());
        }
    }
}
