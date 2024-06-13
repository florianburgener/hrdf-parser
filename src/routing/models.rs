use std::collections::HashSet;

use chrono::NaiveDateTime;

use crate::{models::Journey, storage::DataStorage};

#[derive(Debug, Clone)]
pub struct RouteSection {
    journey_id: Option<i32>,
    departure_stop_id: i32,
    arrival_stop_id: i32,
    arrival_at: NaiveDateTime,
    duration: Option<i16>,
}

impl RouteSection {
    pub fn new(
        journey_id: Option<i32>,
        departure_stop_id: i32,
        arrival_stop_id: i32,
        arrival_at: NaiveDateTime,
        duration: Option<i16>,
    ) -> Self {
        Self {
            journey_id,
            departure_stop_id,
            arrival_stop_id,
            arrival_at,
            duration,
        }
    }

    // Getters/Setters

    pub fn journey_id(&self) -> Option<i32> {
        self.journey_id
    }

    pub fn departure_stop_id(&self) -> i32 {
        self.departure_stop_id
    }

    pub fn arrival_stop_id(&self) -> i32 {
        self.arrival_stop_id
    }

    pub fn set_arrival_stop_id(&mut self, value: i32) {
        self.arrival_stop_id = value;
    }

    pub fn arrival_at(&self) -> NaiveDateTime {
        self.arrival_at
    }

    pub fn set_arrival_at(&mut self, value: NaiveDateTime) {
        self.arrival_at = value;
    }

    pub fn duration(&self) -> Option<i16> {
        self.duration
    }

    // Functions

    pub fn journey<'a>(&'a self, data_storage: &'a DataStorage) -> Option<&Journey> {
        self.journey_id()
            .map(|journey_id| data_storage.journeys().find(journey_id))
    }
}

#[derive(Debug)]
pub struct Route {
    route_sections: Vec<RouteSection>,
    visited_stops: HashSet<i32>,
}

impl Route {
    pub fn new(route_sections: Vec<RouteSection>, visited_stops: HashSet<i32>) -> Self {
        Self {
            route_sections,
            visited_stops,
        }
    }

    pub fn route_sections(&self) -> &Vec<RouteSection> {
        &self.route_sections
    }

    pub fn visited_stops(&self) -> &HashSet<i32> {
        &self.visited_stops
    }

    // Functions

    pub fn last_route_section(&self) -> &RouteSection {
        self.route_sections().last().unwrap()
    }

    pub fn arrival_stop_id(&self) -> i32 {
        self.last_route_section().arrival_stop_id()
    }

    pub fn arrival_at(&self) -> NaiveDateTime {
        self.last_route_section().arrival_at()
    }

    pub fn has_visited_any_stops(&self, stops: &HashSet<i32>) -> bool {
        self.visited_stops().intersection(stops).count() != 0
    }

    pub fn route_sections_with_existing_journey(&self) -> Vec<&RouteSection> {
        self.route_sections()
            .iter()
            .filter(|route_section| route_section.journey_id().is_some())
            .collect()
    }

    pub fn count_connections(&self) -> usize {
        self.route_sections_with_existing_journey().len()
    }
}
