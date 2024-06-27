use std::sync::Arc;

use axum::{
    routing::{get, post},
    Json, Router,
};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    hrdf::Hrdf, isochrone, models::Coordinates, routing::RouteResult, utils::create_date_time,
};

pub async fn run_service(hrdf: Hrdf) {
    println!("Starting the server...");

    let hrdf = Arc::new(hrdf);

    let hrdf_clone_1 = Arc::clone(&hrdf);
    let hrdf_clone_2 = Arc::clone(&hrdf);

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    #[rustfmt::skip]
    let app = Router::new()
        .route(
            "/routing/reachable-stops-within-time-limit",
            post(move |payload| get_reachable_stops_within_time_limit(Arc::clone(&hrdf_clone_1), payload)),
        )
        .route(
            "/isochrones",
            get(move || get_isochrones(Arc::clone(&hrdf_clone_2))),
        )
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8100").await.unwrap();

    println!("Listening on 0.0.0.0:8100...");

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct GetReachableStopsWithinTimeLimitRequest {
    departure_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: NaiveTime,
    time_limit: u32,
}

async fn get_reachable_stops_within_time_limit(
    hrdf: Arc<Hrdf>,
    Json(payload): Json<GetReachableStopsWithinTimeLimitRequest>,
) -> Json<Vec<RouteResult>> {
    println!("{:?}", payload);
    // TODO: Check that the stop exists.

    let routes = hrdf.find_reachable_stops_within_time_limit(
        payload.departure_stop_id,
        NaiveDateTime::new(payload.departure_date, payload.departure_time),
        Duration::hours(payload.time_limit.into()),
        true,
    );
    Json(routes)
}

async fn get_isochrones(hrdf: Arc<Hrdf>) -> Json<Vec<Vec<Coordinates>>> {
    let routes = hrdf.find_reachable_stops_within_time_limit(
        8587418,
        create_date_time(2023, 2, 3, 13, 25),
        Duration::minutes(30),
        false,
    );
    let isochrones = isochrone::get_isochrones(routes);
    Json(isochrones)
}
