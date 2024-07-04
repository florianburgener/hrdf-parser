use chrono::Duration;

use crate::models::Coordinates;

pub fn lv95_to_wgs84(easting: f64, northing: f64) -> (f64, f64) {
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

pub fn distance_between_2_points(point1: Coordinates, point2: Coordinates) -> f64 {
    let x_sqr = (point2.easting() - point1.easting()).powi(2);
    let y_sqr = (point2.northing() - point1.northing()).powi(2);
    (x_sqr + y_sqr).sqrt()
}

pub fn distance_to_time(distance: f64, speed_in_kilometers_per_hour: f64) -> Duration {
    let speed_in_meters_per_second = speed_in_kilometers_per_hour / 3.6;
    Duration::seconds((distance / speed_in_meters_per_second) as i64)
}

pub fn time_to_distance(duration: Duration, speed_in_kilometers_per_hour: f64) -> f64 {
    let speed_in_meters_per_second = speed_in_kilometers_per_hour / 3.6;
    duration.num_seconds() as f64 * speed_in_meters_per_second
}
