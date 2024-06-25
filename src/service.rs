use std::sync::Arc;

use axum::{routing::get, Json, Router};
use chrono::Duration;
use serde::Serialize;

use crate::{hrdf::Hrdf, routing::RouteResult, utils::create_date_time};

pub async fn run_service(hrdf: Hrdf) {
    println!("Starting the server...");

    let hrdf = Arc::new(hrdf);

    let app = Router::new().route("/my_endpoint", get(move || my_endpoint(Arc::clone(&hrdf))));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8100").await.unwrap();

    println!("Listening on 0.0.0.0:8100...");

    axum::serve(listener, app).await.unwrap();
}

async fn my_endpoint(hrdf: Arc<Hrdf>) -> Json<Vec<RouteResult>> {
    let routes = hrdf.find_reachable_stops_within_time_limit(
        8501008,
        create_date_time(2023, 2, 3, 13, 25),
        Duration::hours(2),
        true,
    );
    Json(routes.into_iter().map(|(_, v)| v).collect())

    // println!("{}", hrdf.data_storage().journeys().entries().len());
    // let response = MyResponse {
    //     message: "Hello, world!".to_string(),
    //     code: 200,
    // };
    // Json(response)
}

#[derive(Serialize)]
struct MyResponse {
    message: String,
    code: u32,
}
