use super::handlers::health as health_handler;
use crate::{context::Context, http::filters::with_resources_context};
use std::sync::Arc;
use warp::{self, Filter};

/// GET /health
pub fn healthcheck_route(
    resources_ctx: Arc<Context>,
) -> impl Filter<Extract = impl warp::Reply + 'static, Error = warp::Rejection> + Clone + 'static {
    let healthcheck_route = warp::path!("health")
        .and(with_resources_context(resources_ctx.clone()))
        .and_then(health_handler)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "HEALTHCHECK",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    healthcheck_route
}

/// GET /
pub fn homepage_route(
) -> impl Filter<Extract = impl warp::Reply + 'static, Error = warp::Rejection> + Clone {
    let homepage_route = warp::path::end()
        .map(|| {
            warp::http::Response::builder()
                .header("content-type", "text/html")
                .body(format!(
                    "<html>
                <h1>Orderbook Api</h1>
            </html>"
                ))
        })
        .with(warp::trace(|info| {
            tracing::info_span!(
                "HOMEPAGE",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    homepage_route
}
