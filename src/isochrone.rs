mod circles;
mod constants;
mod contour_line;
mod models;
mod utils;

use crate::isochrone::utils::haversine_distance;
use crate::models::Coordinates;
use crate::models::Stop;
use crate::routing::RouteResult;
use crate::routing::RouteSectionResult;
use crate::storage::DataStorage;
use constants::WALKING_SPEED_IN_KILOMETERS_PER_HOUR;
pub use models::DisplayMode as IsochroneDisplayMode;
pub use models::IsochroneCollection;

use chrono::{Duration, NaiveDateTime};

use models::Isochrone;
use utils::distance_to_time;
use utils::wgs84_to_lv95;

use crate::hrdf::Hrdf;
use crate::models::Model;

impl Hrdf {
    pub fn compute_isochrones(
        &self,
        origin_point_latitude: f64,
        origin_point_longitude: f64,
        departure_at: NaiveDateTime,
        time_limit: Duration,
        isochrone_interval: Duration,
        display_mode: models::DisplayMode,
        verbose: bool,
    ) -> IsochroneCollection {
        let departure_stop = find_nearest_stop(
            self.data_storage(),
            origin_point_latitude,
            origin_point_longitude,
        );
        let departure_stop_coord = departure_stop.wgs84_coordinates().unwrap();

        let (adjusted_departure_at, adjusted_time_limit) = adjust_departure_at(
            departure_at,
            time_limit,
            origin_point_latitude,
            origin_point_longitude,
            &departure_stop,
        );

        let mut routes: Vec<_> = self
            .find_reachable_stops_within_time_limit(
                departure_stop.id(),
                adjusted_departure_at,
                adjusted_time_limit,
                verbose,
            )
            .into_iter()
            .filter(|route| {
                // Keeps only stops in Switzerland.
                let stop_id = route.sections().last().unwrap().arrival_stop_id();
                stop_id.to_string().starts_with("85")
            })
            .collect();

        let (easting, northing) = wgs84_to_lv95(origin_point_latitude, origin_point_longitude);
        let route = RouteResult::new(
            NaiveDateTime::default(),
            departure_at,
            vec![RouteSectionResult::new(
                None,
                0,
                Some(Coordinates::default()),
                Some(Coordinates::default()),
                0,
                Some(Coordinates::new(
                    crate::models::CoordinateSystem::LV95,
                    easting,
                    northing,
                )),
                Some(Coordinates::default()),
                Some(NaiveDateTime::default()),
                Some(NaiveDateTime::default()),
                Some(0),
            )],
        );
        routes.push(route);

        if routes.len() == 0 {
            return IsochroneCollection::new(vec![], departure_stop_coord);
        }

        let grid = if display_mode == models::DisplayMode::ContourLine {
            Some(contour_line::create_grid(&routes, departure_at, time_limit))
        } else {
            None
        };

        let mut isochrones = Vec::new();
        let isochrone_count = time_limit.num_minutes() / isochrone_interval.num_minutes();

        for i in 0..isochrone_count {
            let time_limit = Duration::minutes(isochrone_interval.num_minutes() * (i + 1));

            let polygons = match display_mode {
                IsochroneDisplayMode::Circles => {
                    circles::get_polygons(&routes, departure_at, time_limit)
                }
                IsochroneDisplayMode::ContourLine => {
                    let (grid, num_points_x, num_points_y, min_point) = grid.as_ref().unwrap();
                    contour_line::get_polygons(
                        &grid,
                        *num_points_x,
                        *num_points_y,
                        *min_point,
                        time_limit,
                    )
                }
            };

            isochrones.push(Isochrone::new(polygons, time_limit.num_minutes() as u32));
        }

        IsochroneCollection::new(isochrones, departure_stop_coord)
    }
}

fn find_nearest_stop(
    data_storage: &DataStorage,
    origin_point_latitude: f64,
    origin_point_longitude: f64,
) -> &Stop {
    data_storage
        .stops()
        .entries()
        .into_iter()
        // Only considers stops in Switzerland.
        .filter(|stop| stop.id().to_string().starts_with("85"))
        .filter(|stop| stop.wgs84_coordinates().is_some())
        .min_by(|a, b| {
            let coord_1 = a.wgs84_coordinates().unwrap();
            let distance_1 = haversine_distance(
                origin_point_latitude,
                origin_point_longitude,
                coord_1.latitude(),
                coord_1.longitude(),
            );

            let coord_2 = b.wgs84_coordinates().unwrap();
            let distance_2 = haversine_distance(
                origin_point_latitude,
                origin_point_longitude,
                coord_2.latitude(),
                coord_2.longitude(),
            );

            distance_1.partial_cmp(&distance_2).unwrap()
        })
        // The stop list cannot be empty.
        .unwrap()
}

fn adjust_departure_at(
    departure_at: NaiveDateTime,
    time_limit: Duration,
    origin_point_latitude: f64,
    origin_point_longitude: f64,
    departure_stop: &Stop,
) -> (NaiveDateTime, Duration) {
    let distance = {
        let coord = departure_stop.wgs84_coordinates().unwrap();

        haversine_distance(
            origin_point_latitude,
            origin_point_longitude,
            coord.latitude(),
            coord.longitude(),
        ) * 1000.0
    };

    let duration = distance_to_time(distance, WALKING_SPEED_IN_KILOMETERS_PER_HOUR);

    let adjusted_departure_at = departure_at.checked_add_signed(duration).unwrap();
    let adjusted_time_limit = time_limit - duration;

    (adjusted_departure_at, adjusted_time_limit)
}
