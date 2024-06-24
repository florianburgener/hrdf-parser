use chrono::NaiveDate;
use rustc_hash::FxHashSet;

use crate::{
    models::{Journey, Model},
    storage::DataStorage,
};

use super::{
    models::{Route, RouteSection},
    utils::clone_update_route,
};

impl Route {
    pub fn extend(
        &self,
        data_storage: &DataStorage,
        journey_id: i32,
        date: NaiveDate,
        is_departure_date: bool,
    ) -> Option<Route> {
        let journey = data_storage.journeys().find(journey_id);

        if journey.is_last_stop(self.arrival_stop_id(), false) {
            return None;
        }

        let is_same_journey = self
            .last_section()
            .journey_id()
            .map_or(false, |id| id == journey_id);

        RouteSection::find_next(
            data_storage,
            journey,
            self.arrival_stop_id(),
            date,
            is_departure_date,
        )
        .and_then(|(new_section, new_visited_stops)| {
            if self.has_visited_any_stops(&new_visited_stops)
                && new_section.arrival_stop_id() != journey.first_stop_id()
            {
                return None;
            }

            let new_route = clone_update_route(self, |cloned_sections, cloned_visited_stops| {
                if is_same_journey {
                    let last_section = cloned_sections.last_mut().unwrap();
                    last_section.set_arrival_stop_id(new_section.arrival_stop_id());
                    last_section.set_arrival_at(new_section.arrival_at());
                } else {
                    cloned_sections.push(new_section);
                }

                cloned_visited_stops.extend(new_visited_stops);
            });
            Some(new_route)
        })
    }
}

impl RouteSection {
    pub fn find_next(
        data_storage: &DataStorage,
        journey: &Journey,
        departure_stop_id: i32,
        date: NaiveDate,
        is_departure_date: bool,
    ) -> Option<(RouteSection, FxHashSet<i32>)> {
        let mut route_iter = journey.route().iter();

        while let Some(route_entry) = route_iter.next() {
            if route_entry.stop_id() == departure_stop_id {
                break;
            }
        }

        let mut visited_stops = FxHashSet::default();

        while let Some(route_entry) = route_iter.next() {
            let stop = route_entry.stop(data_storage);
            visited_stops.insert(stop.id());

            if stop.can_be_used_as_exchange_point() || journey.is_last_stop(stop.id(), false) {
                let arrival_at = journey.arrival_at_of_with_origin(
                    stop.id(),
                    date,
                    is_departure_date,
                    departure_stop_id,
                );

                return Some((
                    RouteSection::new(
                        Some(journey.id()),
                        departure_stop_id,
                        stop.id(),
                        arrival_at,
                        None,
                    ),
                    visited_stops,
                ));
            }
        }

        None
    }
}
