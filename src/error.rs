use crate::http::models::{ErrorResponse, FieldError};
use displaydoc::Display as DisplayDoc;
use reqwest::StatusCode;
use std::{convert::Infallible, error::Error as StdError, net::AddrParseError};
use thiserror::Error;
use validator::{ValidationErrors, ValidationErrorsKind};
use warp::{Rejection, Reply};

/// Password hashing error types.
#[derive(Debug, DisplayDoc, Error)]
pub enum Error {
    /// Server parse address error: `{0}`
    ParseAddr(AddrParseError),
    /// Unparsable UUID error: `{0}`
    UnparsableUuid(String),
    /// Missing certificate error
    MissingCertificate,
    /// Request error: `{0}`
    Request(RequestError),
}

impl warp::reject::Reject for Error {}

/// Request-related errors
#[derive(Clone, Debug, DisplayDoc, Error, PartialEq)]
pub enum RequestError {
    /// JSON path error: `{0}`
    JSONPathError(String),
    /// validation error: `{0}`
    ValidationError(ValidationErrors),
}

impl warp::reject::Reject for RequestError {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message, errors) = if err.is_not_found() {
        eprintln!("NOT FOUND error");
        (StatusCode::NOT_FOUND, "Not Found".to_string(), None)
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        eprintln!("Invalid body error");
        (
            StatusCode::BAD_REQUEST,
            e.source()
                .map(|cause| cause.to_string())
                .unwrap_or_else(|| "BAD_REQUEST".to_string()),
            None,
        )
    } else if let Some(Error::UnparsableUuid(e)) = err.find::<Error>() {
        eprintln!("Unparsable uuid error");
        (StatusCode::BAD_REQUEST, e.to_string(), None)
    } else if let Some(Error::Request(e)) = err.find::<Error>() {
        eprintln!("request error: {:?}", e.to_string());
        match e {
            RequestError::JSONPathError(_) => (StatusCode::BAD_REQUEST, e.to_string(), None),
            RequestError::ValidationError(val_errs) => {
                let errors: Vec<FieldError> = val_errs
                    .errors()
                    .iter()
                    .map(|error_kind| FieldError {
                        field: error_kind.0.to_string(),
                        field_errors: match error_kind.1 {
                            ValidationErrorsKind::Struct(struct_err) => {
                                validation_errs_to_str_vec(struct_err)
                            }
                            ValidationErrorsKind::Field(field_errs) => field_errs
                                .iter()
                                .map(|fe| format!("{}: {:?}", fe.code, fe.params))
                                .collect(),
                            ValidationErrorsKind::List(vec_errs) => vec_errs
                                .iter()
                                .map(|ve| {
                                    format!(
                                        "{}: {:?}",
                                        ve.0,
                                        validation_errs_to_str_vec(ve.1).join(" | "),
                                    )
                                })
                                .collect(),
                        },
                    })
                    .collect();
                (
                    StatusCode::BAD_REQUEST,
                    "field errors".to_string(),
                    Some(errors),
                )
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        eprintln!("MethodNotAllowed error");
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
            None,
        )
    } else {
        eprintln!("any other unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
            None,
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        status: code.to_string(),
        message: message.into(),
        errors: errors,
    });

    Ok(warp::reply::with_status(json, code))
}

fn validation_errs_to_str_vec(ve: &ValidationErrors) -> Vec<String> {
    ve.field_errors()
        .iter()
        .map(|fe| {
            format!(
                "{}: errors: {}",
                fe.0,
                fe.1.iter()
                    .map(|ve| format!("{}: {:?}", ve.code, ve.params))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
        .collect()
}
