use anyhow::{Context, Result};
use orderbook_api::grpc::protos::orderbook::{
    orderbook_client::OrderbookClient,
    orderbook_server::{Orderbook, OrderbookServer},
    OrderSide,
};
use orderbook_api::grpc::protos::orderbook::{
    AddOrderRequest, AddOrderResponse, CancelOrderRequest, CancelOrderResponse,
    CreateOrderbookRequest, CreateOrderbookResponse, DeleteOrderbookRequest,
    DeleteOrderbookResponse, GetOrderRequest, GetOrderResponse, Order, OrderStatus, OrderType,
};
use uuid::Uuid;

pub mod tests {

    use super::*;

    #[tokio::test]
    async fn test_orderbook() {
        let mut client = OrderbookClient::connect("http://0.0.0.0:50052")
            .await
            .expect("Server should start");

        // create orderbook
        let ticker = "BTCUSD".to_string();

        // create orderbook
        let create_orderbook_request = tonic::Request::new(CreateOrderbookRequest {
            name: format!("ORDERBOOK_{}", Uuid::new_v4().to_string()),
            ticker: ticker.to_string(),
        });

        let resp = client
            .create_orderbook(create_orderbook_request)
            .await
            .unwrap()
            .into_inner();
        println!("Orderbook creation response {:?}", resp);

        // create sell order
        let order1 = tonic::Request::new(AddOrderRequest {
            orderbook_id: resp.uuid.clone(),
            order_type: OrderType::Market.into(),
            order_side: OrderSide::Bid.into(),
            price: "0".to_string(),
            quantity: "2".to_string(),
        });

        // wants to buy for a given price
        let order2 = tonic::Request::new(AddOrderRequest {
            orderbook_id: resp.uuid.clone(),
            order_type: OrderType::Limit.into(),
            order_side: OrderSide::Ask.into(),
            price: "120".to_string(),
            quantity: "3".to_string(),
        });

        // selling
        let order3 = tonic::Request::new(AddOrderRequest {
            orderbook_id: resp.uuid.clone(),
            order_type: OrderType::Market.into(),
            order_side: OrderSide::Bid.into(),
            price: "0".to_string(),
            quantity: "4".to_string(),
        });

        // create orders
        for order in [order1, order2, order3].into_iter() {
            let resp = client.add_order(order).await.unwrap();
            println!("Order exec response {:?}", resp.into_inner());
        }

        // delete orderbook
        let delete_orderbook_request = tonic::Request::new(DeleteOrderbookRequest {
            uuid: resp.uuid.clone(),
        });

        let resp = client
            .delete_orderbook(delete_orderbook_request)
            .await
            .unwrap();
        println!("Orderbook deletion response {:?}", resp.into_inner());
    }
}
