use reqwest::StatusCode;
use std::sync::Arc;
use warp::Rejection;

use crate::context::Context;

// healthcheck route
pub async fn health(ctx: Arc<Context>) -> Result<impl warp::Reply, Rejection> {
    Ok(StatusCode::OK)
}
