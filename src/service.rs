use std::sync::Arc;

use axum::{routing::post, Json, Router};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use serde::Deserialize;

use crate::{hrdf::Hrdf, routing::RouteResult};

pub async fn run_service(hrdf: Hrdf) {
    println!("Starting the server...");

    let hrdf = Arc::new(hrdf);

    let app = Router::new().route(
        "/find-reachable-stops-within-time-limit",
        // post(move |Form(params): Form<OneToManyRequest>| {
        //     find_reachable_stops_within_time_limit(Arc::clone(&hrdf))
        // }),
        post(
            move |payload: Json<FindReachableStopsWithinTimeLimitRequest>| {
                find_reachable_stops_within_time_limit(Arc::clone(&hrdf), payload)
            },
        ),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8100").await.unwrap();

    println!("Listening on 0.0.0.0:8100...");

    axum::serve(listener, app).await.unwrap();
}

async fn find_reachable_stops_within_time_limit(
    hrdf: Arc<Hrdf>,
    Json(payload): Json<FindReachableStopsWithinTimeLimitRequest>,
) -> Json<Vec<RouteResult>> {
    println!("{:?}", payload);
    // TODO: Check that the stop exists.

    let routes = hrdf.find_reachable_stops_within_time_limit(
        payload.departure_stop_id,
        NaiveDateTime::new(payload.departure_date, payload.departure_time),
        Duration::hours(payload.time_limit.into()),
        true,
    );
    Json(routes.into_iter().map(|(_, v)| v).collect())
}

#[derive(Debug, Deserialize)]
struct FindReachableStopsWithinTimeLimitRequest {
    departure_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: NaiveTime,
    time_limit: u32,
}
