use std::{str::FromStr, sync::Arc};

use axum::{extract::Query, routing::get, Json, Router};
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime};
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    hrdf::Hrdf,
    isochrone::{IsochroneCollection, IsochroneDisplayMode},
};

pub async fn run_service(hrdf: Hrdf) {
    println!("Starting the server...");

    let hrdf = Arc::new(hrdf);
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    #[rustfmt::skip]
    let app = Router::new()
        .route(
            "/isochrones",
            get(move |params| compute_isochrones(Arc::clone(&hrdf), params)),
        )
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8100").await.unwrap();

    println!("Listening on 0.0.0.0:8100...");

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct ComputeIsochronesRequest {
    departure_stop_id: i32,
    departure_date: NaiveDate,
    departure_time: NaiveTime,
    time_limit: u32,
    isochrone_interval: u32,
    display_mode: String,
}

async fn compute_isochrones(
    hrdf: Arc<Hrdf>,
    Query(params): Query<ComputeIsochronesRequest>,
) -> Json<IsochroneCollection> {
    let isochrones = hrdf.compute_isochrones(
        params.departure_stop_id,
        NaiveDateTime::new(params.departure_date, params.departure_time),
        Duration::minutes(params.time_limit.into()),
        Duration::minutes(params.isochrone_interval.into()),
        IsochroneDisplayMode::from_str(&params.display_mode).unwrap(),
        false,
    );
    Json(isochrones)
}
