use super::error::GrpcError;
use super::handler::{
    add_order, cancel_order, create_orderbook, delete_orderbook, get_order, get_stats,
};
use super::protos::orderbook::orderbook_server::{Orderbook, OrderbookServer};
use super::protos::orderbook::{
    AddOrderRequest, AddOrderResponse, CancelOrderRequest, CancelOrderResponse,
    CreateOrderbookRequest, CreateOrderbookResponse, DeleteOrderbookRequest,
    DeleteOrderbookResponse, GetOrderRequest, GetOrderResponse, GetStatsRequest, GetStatsResponse,
    FILE_DESCRIPTOR_SET,
};
use crate::config::{GrpcConfig, OrderbookConfig};
use crate::context::Context;
use futures::FutureExt;
use lobster::OrderBook;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::oneshot;
use tonic::codegen::CompressionEncoding;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Debug, Default)]
pub struct CustomOrderbook {
    id: String,
    orderbook: OrderBook,
    ticker: String,
    creator: Option<String>,
    name: String,
}

impl CustomOrderbook {
    pub fn new(
        id: String,
        config: OrderbookConfig,
        ticker: String,
        creator: Option<String>,
        name: String,
    ) -> Self {
        Self {
            id,
            orderbook: OrderBook::new(config.arena_capacity, config.queue_capacity, true),
            ticker,
            creator,
            name,
        }
    }

    pub fn get_uuid(&self) -> &str {
        &self.id
    }

    pub fn get_orderbook(&self) -> &OrderBook {
        &self.orderbook
    }

    pub fn get_orderbook_mut(&mut self) -> &mut OrderBook {
        &mut self.orderbook
    }

    pub fn get_ticker(&self) -> &str {
        &self.ticker
    }

    pub fn get_creator(&self) -> &Option<String> {
        &self.creator
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone)]
pub struct OrderbookGrpcService {
    pub resources_ctx: Arc<Context>,
    // maps uuid to orderbook instance
    pub orderbook_client: Arc<Mutex<BTreeMap<String, CustomOrderbook>>>,
    pub orderbook_config: OrderbookConfig,
}

impl OrderbookGrpcService {
    pub fn new(orderbook_config: OrderbookConfig, resources_ctx: Arc<Context>) -> Self {
        Self {
            resources_ctx,
            orderbook_client: Default::default(),
            orderbook_config,
        }
    }

    pub async fn serve_async(&self, config: GrpcConfig) -> Result<StopHandle, GrpcError> {
        let (shutdown_send, shutdown_recv) = oneshot::channel::<()>();

        // create a server builder
        let mut server_builder = Server::builder()
            .concurrency_limit_per_connection(config.concurrency_limit_per_connection)
            .timeout(config.timeout)
            .initial_stream_window_size(config.initial_stream_window_size)
            .initial_connection_window_size(config.initial_connection_window_size)
            .max_concurrent_streams(config.max_concurrent_streams)
            .tcp_keepalive(config.tcp_keepalive)
            .tcp_nodelay(config.tcp_nodelay)
            .http2_keepalive_interval(config.http2_keepalive_interval)
            .http2_keepalive_timeout(config.http2_keepalive_timeout)
            .http2_adaptive_window(config.http2_adaptive_window)
            .max_frame_size(config.max_frame_size);

        // build the orderbook server
        let orderbook_service =
            OrderbookGrpcService::new(self.orderbook_config.clone(), self.resources_ctx.clone());
        let mut orderbook_server = OrderbookServer::new(orderbook_service)
            .max_decoding_message_size(config.max_decoding_message_size)
            .max_encoding_message_size(config.max_encoding_message_size);

        if let Some(encoding) = &config.send_compressed {
            if encoding.eq_ignore_ascii_case("Gzip") {
                orderbook_server = orderbook_server.send_compressed(CompressionEncoding::Gzip)
            }
        }

        if let Some(encoding) = &config.accept_compressed {
            if encoding.eq_ignore_ascii_case("Gzip") {
                orderbook_server = orderbook_server.accept_compressed(CompressionEncoding::Gzip);
            }
        }

        // now add the orderbook_server to the builder
        let mut router = server_builder.add_service(orderbook_server);

        // add health service
        let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<OrderbookServer<OrderbookGrpcService>>()
            .await;

        let mut health_reporter = health_reporter.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                health_reporter
                    .set_serving::<OrderbookServer<OrderbookGrpcService>>()
                    .await;
            }
        });

        // if reflection, add the reflection server to the builder
        if config.enable_reflection {
            let reflection_service = tonic_reflection::server::Builder::configure()
                .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
                .build()
                .map_err(|e| GrpcError::ReflectionServer(e))?;

            router = router.add_service(reflection_service);

            router = router.add_service(health_service);
        }

        // spawn the grpc server with a stop signal
        tokio::spawn(router.serve_with_shutdown(config.bind, shutdown_recv.map(drop)));

        Ok(StopHandle {
            stop_cmd_sender: shutdown_send,
        })
    }
}

/// Used to be able to stop the gRPC API
pub struct StopHandle {
    stop_cmd_sender: oneshot::Sender<()>,
}

impl StopHandle {
    /// stop the gRPC API gracefully
    pub fn stop(self) {
        if let Err(e) = self.stop_cmd_sender.send(()) {
            tracing::error!("Grpc Api thread panicked: {:?}", e);
        } else {
            tracing::info!("Grpc Api finished cleanly");
        }
    }
}

#[tonic::async_trait]
impl Orderbook for OrderbookGrpcService {
    async fn create_orderbook(
        &self,
        request: Request<CreateOrderbookRequest>,
    ) -> Result<Response<CreateOrderbookResponse>, Status> {
        match create_orderbook(self, request) {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(e.into()),
        }
    }

    async fn delete_orderbook(
        &self,
        request: Request<DeleteOrderbookRequest>,
    ) -> Result<Response<DeleteOrderbookResponse>, Status> {
        match delete_orderbook(self, request) {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(e.into()),
        }
    }

    async fn add_order(
        &self,
        request: Request<AddOrderRequest>,
    ) -> Result<Response<AddOrderResponse>, Status> {
        match add_order(self, request) {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(e.into()),
        }
    }

    async fn cancel_order(
        &self,
        request: Request<CancelOrderRequest>,
    ) -> Result<Response<CancelOrderResponse>, Status> {
        match cancel_order(self, request) {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(e.into()),
        }
    }

    async fn get_order(
        &self,
        request: Request<GetOrderRequest>,
    ) -> Result<Response<GetOrderResponse>, Status> {
        match get_order(self, request) {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(e.into()),
        }
    }

    async fn get_stats(
        &self,
        request: Request<GetStatsRequest>,
    ) -> Result<Response<GetStatsResponse>, Status> {
        match get_stats(self, request) {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(e.into()),
        }
    }
}
