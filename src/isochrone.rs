use chrono::{Duration, NaiveDateTime};
use serde::Serialize;

use crate::{
    hrdf::Hrdf,
    models::{CoordinateSystem, Coordinates},
};

use std::f64::consts::PI;

impl Hrdf {
    pub fn get_isochrones(
        &self,
        departure_stop_id: i32,
        departure_at: NaiveDateTime,
        isochrone_interval: Duration,
        time_limit: Duration,
        verbose: bool,
    ) -> IsochroneCollection {
        const WALKING_SPEED_IN_KILOMETERS_PER_HOUR: f64 = 4.0;

        let routes = self.find_reachable_stops_within_time_limit(
            departure_stop_id,
            departure_at,
            time_limit,
            verbose,
        );

        let mut isochrones = Vec::new();
        let isochrone_count = time_limit.num_minutes() / isochrone_interval.num_minutes();

        for i in 0..isochrone_count {
            let time_limit_i = Duration::minutes(isochrone_interval.num_minutes() * (i + 1));

            let isochrone = routes
                .iter()
                .filter_map(|route| {
                    let lv95 = route
                        .sections()
                        .last()
                        .unwrap()
                        .arrival_stop_lv95_coordinates();

                    let duration = route.arrival_at() - departure_at;
                    // if route.sections().last().unwrap().arrival_stop_id() == 8774538 {
                    //     println!("{i} {} {}", duration, route.arrival_at());
                    // }
                    lv95.zip(Some(duration))
                })
                .filter(|&(_, duration)| duration <= time_limit_i)
                .map(|(center_lv95, duration)| {
                    // let distance = f64::min(500.0, time_to_distance(duration, WALKING_SPEED_IN_KILOMETERS_PER_HOUR));
                    let distance = time_to_distance(
                        time_limit_i - duration,
                        WALKING_SPEED_IN_KILOMETERS_PER_HOUR,
                    );

                    generate_lv95_circle_points(
                        center_lv95.easting(),
                        center_lv95.northing(),
                        distance,
                        18,
                    )
                    .into_iter()
                    .map(|lv95| {
                        let wgs84 = lv95_to_wgs84(lv95.easting(), lv95.northing());
                        Coordinates::new(CoordinateSystem::WGS84, wgs84.0, wgs84.1)
                    })
                    .collect()
                })
                .collect();

            isochrones.push(Isochrone::new(isochrone, time_limit_i.num_minutes() as u32));
        }

        let departure_stop = self.data_storage().stops().find(departure_stop_id);
        let departure_stop_coordinates = departure_stop.wgs84_coordinates().unwrap();
        IsochroneCollection::new(isochrones, departure_stop_coordinates)
    }
}

#[derive(Debug, Serialize)]
pub struct IsochroneCollection {
    items: Vec<Isochrone>,
    departure_stop_coordinates: Coordinates,
}

impl IsochroneCollection {
    pub fn new(isochrones: Vec<Isochrone>, departure_stop_coordinates: Coordinates) -> Self {
        Self {
            items: isochrones,
            departure_stop_coordinates,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Isochrone {
    polygons: Vec<Vec<Coordinates>>,
    time_limit: u32, // In minutes.
}

impl Isochrone {
    pub fn new(polygons: Vec<Vec<Coordinates>>, time_limit: u32) -> Self {
        Self {
            polygons,
            time_limit,
        }
    }
}

fn generate_lv95_circle_points(e: f64, n: f64, radius: f64, num_points: usize) -> Vec<Coordinates> {
    let mut points = Vec::new();
    let angle_step = 2.0 * PI / num_points as f64;

    for i in 0..num_points {
        let angle = i as f64 * angle_step;
        let de = radius * angle.cos();
        let dn = radius * angle.sin();
        points.push(Coordinates::new(CoordinateSystem::LV95, e + de, n + dn));
    }

    points
}

fn time_to_distance(duration: Duration, speed_in_kilometers_per_hour: f64) -> f64 {
    let speed_in_meters_per_second = speed_in_kilometers_per_hour / 3.6;
    duration.num_seconds() as f64 * speed_in_meters_per_second
}

fn lv95_to_wgs84(easting: f64, northing: f64) -> (f64, f64) {
    // Convert LV95 to LV03
    let e_lv03 = easting - 2_000_000.0;
    let n_lv03 = northing - 1_000_000.0;

    // Auxiliary values
    let e_aux = (e_lv03 - 600_000.0) / 1_000_000.0;
    let n_aux = (n_lv03 - 200_000.0) / 1_000_000.0;

    // Calculate latitude (in WGS84)
    let lat = 16.9023892 + 3.238272 * n_aux
        - 0.270978 * e_aux.powi(2)
        - 0.002528 * n_aux.powi(2)
        - 0.0447 * e_aux.powi(2) * n_aux
        - 0.0140 * n_aux.powi(3);

    // Calculate longitude (in WGS84)
    let lon =
        2.6779094 + 4.728982 * e_aux + 0.791484 * e_aux * n_aux + 0.1306 * e_aux * n_aux.powi(2)
            - 0.0436 * e_aux.powi(3);

    // Convert from degrees to WGS84
    let lat_wgs84 = lat * 100.0 / 36.0;
    let lon_wgs84 = lon * 100.0 / 36.0;

    (lat_wgs84, lon_wgs84)
}
