use reqwest::Method;
use std::{convert::Infallible, sync::Arc};
use warp::filters::cors::Builder;
use warp::Filter;

use crate::context::Context;

pub fn with_cors() -> Builder {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Sec-Fetch-Mode",
            "Sec-Fetch-Dest",
            "Sec-Fetch-Site",
            "Mode",
            "Credentials",
            "content-type",
            reqwest::header::ACCEPT.as_str(),
            reqwest::header::ACCEPT_CHARSET.as_str(),
            reqwest::header::ACCEPT_ENCODING.as_str(),
            reqwest::header::ACCEPT_LANGUAGE.as_str(),
            reqwest::header::ACCEPT_RANGES.as_str(),
            reqwest::header::USER_AGENT.as_str(),
            reqwest::header::REFERER.as_str(),
            reqwest::header::REFERRER_POLICY.as_str(),
            reqwest::header::ORIGIN.as_str(),
            reqwest::header::ALLOW.as_str(),
            reqwest::header::COOKIE.as_str(),
            reqwest::header::HOST.as_str(),
            reqwest::header::ACCESS_CONTROL_REQUEST_METHOD.as_str(),
            reqwest::header::ACCESS_CONTROL_REQUEST_HEADERS.as_str(),
            reqwest::header::ACCESS_CONTROL_EXPOSE_HEADERS.as_str(),
            reqwest::header::ACCESS_CONTROL_MAX_AGE.as_str(),
            reqwest::header::ACCESS_CONTROL_ALLOW_METHODS.as_str(),
            reqwest::header::ACCESS_CONTROL_ALLOW_CREDENTIALS.as_str(),
            reqwest::header::ACCESS_CONTROL_ALLOW_ORIGIN.as_str(),
            reqwest::header::ACCESS_CONTROL_ALLOW_HEADERS.as_str(),
            reqwest::header::CONTENT_TYPE.as_str(),
            reqwest::header::AUTHORIZATION.as_str(),
            reqwest::header::UPGRADE.as_str(),
            reqwest::header::UPGRADE_INSECURE_REQUESTS.as_str(),
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS,
            Method::PUT,
        ]);

    cors
}

pub fn with_resources_context(
    resources_ctx: Arc<Context>,
) -> impl warp::Filter<Extract = (Arc<Context>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&resources_ctx))
}
