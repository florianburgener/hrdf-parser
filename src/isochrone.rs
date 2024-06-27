use delaunator::{triangulate, Point};
use itertools::Itertools;
use ndarray::*;
use ndarray_linalg::*;
use std::collections::{HashMap, HashSet};

use crate::{
    models::{CoordinateSystem, Coordinates},
    routing::RouteResult,
};

// https://github.com/mblackgeo/fast-alphashape

/// Calculate the circumcentre of a set of points in barycentric coordinates.
pub fn circumcentre(points: ArrayView2<f64>) -> Array1<f64> {
    let n_rows = points.shape()[0];

    // Build the Coefficient matrix
    let matrix = concatenate![
        Axis(0),
        concatenate![
            Axis(1),
            2.0 * points.dot(&points.t()),
            Array::ones((n_rows, 1))
        ],
        concatenate![Axis(1), Array::ones((1, n_rows)), Array::zeros((1, 1))]
    ];

    // build the ordinate
    let ord = concatenate![
        Axis(0),
        (&points * &points).sum_axis(Axis(1)),
        Array::ones(1)
    ];

    // solve
    // TODO error handling here for failure to converge
    let res = matrix.solve_into(ord).unwrap();
    res.slice(s![..-1]).to_owned()
}

/// Calculate the circumradius of a given set of points
pub fn circumradius(points: ArrayView2<f64>) -> f64 {
    let slice = points.slice(s![0, ..]).to_owned();
    let centre = circumcentre(points.view());

    (slice - centre.dot(&points)).norm()
}

/// Returns simplices of the given set of points
pub fn get_triangles(points: ArrayView2<f64>) -> Vec<Vec<usize>> {
    let pts: Vec<Point> = points
        .axis_iter(Axis(0))
        .into_iter()
        .map(|arr| Point {
            x: arr[0],
            y: arr[1],
        })
        .collect();

    triangulate(&pts)
        .triangles
        .chunks_exact(3)
        .map(|x| x.to_vec())
        .collect()
}

/// Get the indices of each valid simplex
pub fn get_edges(points: ArrayView2<f64>, tri: &[usize], alpha: f64) -> Vec<Vec<usize>> {
    // extract the coordinates and circumradius for this simplex triangle
    let coords = stack![
        Axis(0),
        points.slice(s![tri[0], ..]),
        points.slice(s![tri[1], ..]),
        points.slice(s![tri[2], ..]),
    ];
    let rad = circumradius(coords.view());

    // extract the indices of each point of the simplex
    let mut idxs: Vec<usize> = Vec::new();
    for c in coords.rows() {
        for (idx, p) in points.rows().into_iter().enumerate() {
            if p == c {
                idxs.push(idx);
            }
        }
    }

    // add the points to edges if required
    let mut edges = Vec::new();
    if rad < 1.0 / alpha {
        for edge in idxs.into_iter().combinations(2) {
            edges.push(edge);
        }
    }

    edges
}

pub fn find_exterior_edges(edges: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut edge_count = HashMap::new();

    // Count the occurrences of each edge
    for edge in &edges {
        let sorted_edge = if edge[0] < edge[1] {
            edge.clone()
        } else {
            vec![edge[1], edge[0]]
        };
        *edge_count.entry(sorted_edge).or_insert(0) += 1;
    }

    // Collect edges that appear only once (exterior edges)
    let mut exterior_edges: Vec<Vec<usize>> = edge_count
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(edge, _)| edge)
        .collect();

    // Use a HashSet to store visited edges and a Vec to store the sorted edges
    let mut sorted_edges = Vec::new();
    let mut visited = HashSet::new();

    // Start with the first edge
    if let Some(start_edge) = exterior_edges.pop() {
        sorted_edges.push(start_edge.clone());
        visited.insert(start_edge);

        // Sort the edges by connecting them in order
        while !exterior_edges.is_empty() {
            let last_edge = sorted_edges.last().unwrap();
            let next_vertex = last_edge[1];

            // Find the next edge that connects to the last vertex
            if let Some(pos) = exterior_edges
                .iter()
                .position(|edge| edge[0] == next_vertex || edge[1] == next_vertex)
            {
                let mut next_edge = exterior_edges.remove(pos);

                // Ensure the edge is oriented correctly
                if next_edge[0] != next_vertex {
                    next_edge.reverse();
                }

                sorted_edges.push(next_edge.clone());
                visited.insert(next_edge);
            } else {
                break;
            }
        }
    }

    sorted_edges
}

// Return the indices of the array that form the edges of the 2D alpha shape
pub fn compute_alphashape(points: ArrayView2<f64>, alpha: f64) -> Vec<Vec<usize>> {
    let edges = get_triangles(points.view())
        .iter()
        .map(|triangle| get_edges(points, triangle, alpha))
        .flatten()
        .collect();

    let aaa = find_exterior_edges(edges);
    aaa
}

fn normalize(arr: &Array2<f64>) -> Array2<f64> {
    let mut normalized_arr = arr.clone();

    for mut column in normalized_arr.columns_mut() {
        let min = column.fold(f64::INFINITY, |a, &b| a.min(b));
        let max = column.fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        if min != max {
            column.mapv_inplace(|x| (x - min) / (max - min));
        } else {
            column.fill(0.0);
        }
    }

    normalized_arr
}

use std::f64::consts::PI;

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

fn get_isochrone(coordinates: Vec<(Coordinates, Coordinates)>) -> Vec<Coordinates> {
    let coordinates: Vec<(Coordinates, Coordinates)> = coordinates
        .into_iter()
        .map(|(lv95, wgs84)| {
            let mut a: Vec<_> =
                generate_lv95_circle_points(lv95.easting(), lv95.northing(), 1000.0, 18)
                    .into_iter()
                    .map(|lv95_i| {
                        let wgs84_i = lv95_to_wgs84(lv95_i.easting(), lv95_i.northing());
                        (
                            lv95_i,
                            Coordinates::new(CoordinateSystem::WGS84, wgs84_i.0, wgs84_i.1),
                        )
                    })
                    .collect();
            a.push((lv95, wgs84));
            a
        })
        .flatten()
        .collect();

    let points = Array2::from_shape_vec(
        (coordinates.len(), 2),
        coordinates
            .iter()
            .map(|(lv95, _)| [lv95.easting(), lv95.northing()])
            .flatten()
            .collect(),
    )
    .unwrap();

    let points = normalize(&points);
    println!("{}", points.len());
    // 380 => 5

    let edges = compute_alphashape(points.view(), 25.);

    edges
        .iter()
        .map(|vertices| coordinates[vertices[0]].1)
        .collect()
}

pub fn get_isochrones(routes: Vec<RouteResult>) -> Vec<Vec<Coordinates>> {
    let coordinates = routes
        .iter()
        .filter_map(|route| {
            let last_section = route.sections().last().unwrap();

            let Some(lv95_coordinates) = last_section.arrival_stop_lv95_coordinates() else {
                return None;
            };

            let Some(wgs84_coordinates) = last_section.arrival_stop_wgs84_coordinates() else {
                return None;
            };

            Some((lv95_coordinates, wgs84_coordinates))
        })
        .collect();

    vec![get_isochrone(coordinates)]
}
