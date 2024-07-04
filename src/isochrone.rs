mod circles;
mod constants;
mod contour_line;
mod models;
mod utils;

pub use models::DisplayMode as IsochroneDisplayMode;
pub use models::IsochroneCollection;

use chrono::{Duration, NaiveDateTime};

use models::Isochrone;

use crate::hrdf::Hrdf;

impl Hrdf {
    pub fn compute_isochrones(
        &self,
        departure_stop_id: i32,
        departure_at: NaiveDateTime,
        time_limit: Duration,
        isochrone_interval: Duration,
        display_mode: models::DisplayMode,
        verbose: bool,
    ) -> IsochroneCollection {
        let routes = self.find_reachable_stops_within_time_limit(
            departure_stop_id,
            departure_at,
            time_limit,
            verbose,
        );

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

        let departure_stop = self.data_storage().stops().find(departure_stop_id);
        let departure_stop_coord = departure_stop.wgs84_coordinates().unwrap();
        IsochroneCollection::new(isochrones, departure_stop_coord)
    }
}
