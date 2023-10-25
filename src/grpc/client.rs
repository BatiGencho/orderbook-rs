use super::error::GrpcError;
use super::protos::orderbook::orderbook_client::OrderbookClient;
use super::protos::orderbook::{
    AddOrderRequest, AddOrderResponse, CancelOrderRequest, CancelOrderResponse,
    CreateOrderbookRequest, CreateOrderbookResponse, DeleteOrderbookRequest,
    DeleteOrderbookResponse, GetOrderRequest, GetOrderResponse, GetStatsRequest, GetStatsResponse,
};
use crate::config::GrpcConfig;

pub struct OrderbookGrpcClient {
    client: OrderbookClient<tonic::transport::channel::Channel>,
}

impl OrderbookGrpcClient {
    pub async fn new(config: &GrpcConfig) -> Result<Self, GrpcError> {
        let addr = format!(
            "http://{}:{}",
            config.bind.ip().to_string(),
            config.bind.port()
        );
        let client = OrderbookClient::connect(addr)
            .await
            .map_err(GrpcError::Transport)?;

        Ok(Self { client })
    }

    async fn create_orderbook(
        &mut self,
        request: tonic::Request<CreateOrderbookRequest>,
    ) -> Result<CreateOrderbookResponse, GrpcError> {
        match self.client.create_orderbook(request).await {
            Ok(response) => {
                let response = response.into_inner();
                return Ok(response);
            }
            Err(status) => return Err(GrpcError::Call(status)),
        }
    }

    async fn delete_orderbook(
        &mut self,
        request: tonic::Request<DeleteOrderbookRequest>,
    ) -> Result<DeleteOrderbookResponse, GrpcError> {
        match self.client.delete_orderbook(request).await {
            Ok(response) => {
                let response = response.into_inner();
                return Ok(response);
            }
            Err(status) => return Err(GrpcError::Call(status)),
        }
    }

    pub async fn add_order(
        &mut self,
        request: tonic::Request<AddOrderRequest>,
    ) -> Result<AddOrderResponse, GrpcError> {
        match self.client.add_order(request).await {
            Ok(response) => {
                let response = response.into_inner();
                return Ok(response);
            }
            Err(status) => return Err(GrpcError::Call(status)),
        }
    }

    async fn cancel_order(
        &mut self,
        request: tonic::Request<CancelOrderRequest>,
    ) -> Result<CancelOrderResponse, GrpcError> {
        match self.client.cancel_order(request).await {
            Ok(response) => {
                let response = response.into_inner();
                return Ok(response);
            }
            Err(status) => return Err(GrpcError::Call(status)),
        }
    }

    async fn get_order(
        &mut self,
        request: tonic::Request<GetOrderRequest>,
    ) -> Result<GetOrderResponse, GrpcError> {
        match self.client.get_order(request).await {
            Ok(response) => {
                let response = response.into_inner();
                return Ok(response);
            }
            Err(status) => return Err(GrpcError::Call(status)),
        }
    }

    async fn get_stats(
        &mut self,
        request: tonic::Request<GetStatsRequest>,
    ) -> Result<GetStatsResponse, GrpcError> {
        match self.client.get_stats(request).await {
            Ok(response) => {
                let response = response.into_inner();
                return Ok(response);
            }
            Err(status) => return Err(GrpcError::Call(status)),
        }
    }
}
