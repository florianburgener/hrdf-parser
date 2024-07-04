use chrono::{Duration, NaiveDateTime};
use contour::ContourBuilder;

use crate::{
    models::{CoordinateSystem, Coordinates},
    routing::RouteResult,
};

use super::{
    constants::{GRID_SPACING_IN_METERS, WALKING_SPEED_IN_KILOMETERS_PER_HOUR},
    utils::{distance_between_2_points, distance_to_time, lv95_to_wgs84, time_to_distance},
};

use rayon::prelude::*;

pub fn create_grid(
    routes: &Vec<RouteResult>,
    departure_at: NaiveDateTime,
    time_limit: Duration,
) -> (Vec<(Coordinates, Duration)>, usize, usize, (f64, f64)) {
    let data: Vec<_> = routes
        .iter()
        .filter_map(|route| {
            let coord = route
                .sections()
                .last()
                .unwrap()
                .arrival_stop_lv95_coordinates();

            let duration = route.arrival_at() - departure_at;
            coord.zip(Some(duration))
        })
        .collect();

    let (min_point, max_point) = get_bounding_box(&data, time_limit);

    let mut grid = Vec::new();

    let num_points_x = ((max_point.0 - min_point.0) / GRID_SPACING_IN_METERS).ceil() as usize;
    let num_points_y = ((max_point.1 - min_point.1) / GRID_SPACING_IN_METERS).ceil() as usize;

    let mut y = min_point.1;
    for _ in 0..num_points_y {
        let mut x = min_point.0;

        for _ in 0..num_points_x {
            grid.push(Coordinates::new(CoordinateSystem::LV95, x, y));
            x += GRID_SPACING_IN_METERS;
        }

        y += GRID_SPACING_IN_METERS;
    }

    let grid = grid
        .par_iter()
        .map(|&coord1| {
            let duration = data
                .par_iter()
                .map(|&(coord2, duration)| {
                    let distance = distance_between_2_points(coord1, coord2);

                    duration + distance_to_time(distance, WALKING_SPEED_IN_KILOMETERS_PER_HOUR)
                })
                .min()
                .unwrap();
            (coord1, duration)
        })
        .collect();
    (grid, num_points_x, num_points_y, min_point)
}

fn get_bounding_box(
    data: &Vec<(Coordinates, Duration)>,
    time_limit: Duration,
) -> ((f64, f64), (f64, f64)) {
    let min_x = data
        .iter()
        .fold(f64::INFINITY, |result, &(coord, duration)| {
            let candidate = coord.easting()
                - time_to_distance(time_limit - duration, WALKING_SPEED_IN_KILOMETERS_PER_HOUR);
            f64::min(result, candidate)
        });

    let max_x = data
        .iter()
        .fold(f64::NEG_INFINITY, |result, &(coord, duration)| {
            let candidate = coord.easting()
                + time_to_distance(time_limit - duration, WALKING_SPEED_IN_KILOMETERS_PER_HOUR);
            f64::max(result, candidate)
        });

    let min_y = data
        .iter()
        .fold(f64::INFINITY, |result, &(coord, duration)| {
            let candidate = coord.northing()
                - time_to_distance(time_limit - duration, WALKING_SPEED_IN_KILOMETERS_PER_HOUR);
            f64::min(result, candidate)
        });

    let max_y = data
        .iter()
        .fold(f64::NEG_INFINITY, |result, &(coord, duration)| {
            let candidate = coord.northing()
                + time_to_distance(time_limit - duration, WALKING_SPEED_IN_KILOMETERS_PER_HOUR);
            f64::max(result, candidate)
        });

    ((min_x, min_y), (max_x, max_y))
}

pub fn get_polygons(
    grid: &Vec<(Coordinates, Duration)>,
    num_points_x: usize,
    num_points_y: usize,
    min_point: (f64, f64),
    time_limit: Duration,
) -> Vec<Vec<Coordinates>> {
    let values: Vec<_> = grid
        .iter()
        .map(
            |&(_, duration)| {
                if duration <= time_limit {
                    1.0
                } else {
                    0.0
                }
            },
        )
        .collect();

    let contour_builder = ContourBuilder::new(num_points_x, num_points_y, false);
    let contours = contour_builder.contours(&values, &[0.5]).unwrap();

    contours[0]
        .geometry()
        .0
        .iter()
        .map(|polygon| {
            polygon
                .exterior()
                .into_iter()
                .map(|coord| {
                    let lv95 = (
                        min_point.0 + GRID_SPACING_IN_METERS * coord.x,
                        min_point.1 + GRID_SPACING_IN_METERS * coord.y,
                    );
                    let wgs84 = lv95_to_wgs84(lv95.0, lv95.1);
                    Coordinates::new(CoordinateSystem::WGS84, wgs84.0, wgs84.1)
                })
                .collect()
        })
        .collect()
}
