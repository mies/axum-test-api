use axum::{
    body::Body,
    extract::{FromRef, FromRequestParts, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use sqlx::postgres::{PgPool, PgPoolOptions};

use autometrics::{autometrics, encode_global_metrics, global_metrics_exporter};

use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() {
    println!("Hell, world!");
}
