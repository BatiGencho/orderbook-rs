use crate::sqlx::error::Error as DbError;
use displaydoc::Display as DisplayDoc;
use std::num::ParseIntError;
use thiserror::Error;

/// grpc-related errors
#[non_exhaustive]
#[derive(Debug, DisplayDoc, Error)]
pub enum OrderbookError {
    /// unparsable order quantity: `{0}`
    UnparsableOrderQuantity(ParseIntError),
    /// unparsable order price: `{0}`
    UnparsableOrderPrice(ParseIntError),
    /// unparsable order side
    UnparsableOrderSide,
    /// unparsable order id
    UnparsableOrderId(ParseIntError),
    /// unknown order type
    UnknownOrderType,
    /// Non-existent orderbook id
    NonExistentOrderbookUuid,
    /// Non-existent order id
    NonExistentOrderId,
    /// orderbook lock error
    LockError,
    /// Wrong OrderId Cancellation
    WrongOrderIdCancel,
    /// uuid parse error
    Uuid(uuid::Error),
}

/// grpc-related errors
#[non_exhaustive]
#[derive(Debug, DisplayDoc, Error)]
pub enum GrpcError {
    /// tonic transport error: `{0}`
    Transport(tonic::transport::Error),
    /// tonic call error: `{0}`
    Call(tonic::Status),
    /// tonic reflection server error: `{0}`
    ReflectionServer(tonic_reflection::server::Error),
    /// `orderbook` error: {0}
    Orderbook(#[from] OrderbookError),
    /// db error: `{0}`
    Database(DbError),
}

impl warp::reject::Reject for GrpcError {}

impl From<GrpcError> for tonic::Status {
    fn from(value: GrpcError) -> Self {
        match value {
            GrpcError::Call(status) => status,
            GrpcError::Transport(error) => {
                tonic::Status::internal(format!("Error: Transport error: {:?}", error.to_string()))
            }
            GrpcError::ReflectionServer(error) => tonic::Status::internal(format!(
                "Error: Reflection Server error: {:?}",
                error.to_string()
            )),
            GrpcError::Database(error) => tonic::Status::internal(format!(
                "Error: Database Server error: {:?}",
                error.to_string()
            )),
            GrpcError::Orderbook(OrderbookError::UnparsableOrderPrice(error)) => {
                tonic::Status::failed_precondition(format!(
                    "Error: unparsable order price {:?}",
                    error.to_string()
                ))
            }
            GrpcError::Orderbook(OrderbookError::UnparsableOrderQuantity(error)) => {
                tonic::Status::failed_precondition(format!(
                    "Error: unparsable order quantity {:?}",
                    error.to_string()
                ))
            }
            GrpcError::Orderbook(OrderbookError::Uuid(error)) => {
                tonic::Status::failed_precondition(format!(
                    "Error: unparsable uuid {:?}",
                    error.to_string()
                ))
            }
            GrpcError::Orderbook(OrderbookError::UnknownOrderType) => {
                tonic::Status::failed_precondition("Error: Unknown order type")
            }
            GrpcError::Orderbook(OrderbookError::NonExistentOrderId) => {
                tonic::Status::failed_precondition("Error: None-existent order id")
            }
            GrpcError::Orderbook(OrderbookError::NonExistentOrderbookUuid) => {
                tonic::Status::failed_precondition("Error: None-existent orderbook id")
            }
            GrpcError::Orderbook(OrderbookError::UnparsableOrderSide) => {
                tonic::Status::failed_precondition("Error: Unparsable order side")
            }
            GrpcError::Orderbook(OrderbookError::WrongOrderIdCancel) => {
                tonic::Status::internal("Error: Wrong Order Id Cancellation")
            }
            _ => tonic::Status::internal("Error: Unhandled Internal error"),
        }
    }
}
