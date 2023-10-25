use super::error::GrpcError;
use super::error::OrderbookError;
use super::protos::orderbook::CreateOrderbookRequest;
use super::protos::orderbook::CreateOrderbookResponse;
use super::protos::orderbook::DeleteOrderbookRequest;
use super::protos::orderbook::DeleteOrderbookResponse;
use super::protos::orderbook::{
    AddOrderRequest, AddOrderResponse, CancelOrderRequest, CancelOrderResponse, FillMetadata,
    GetOrderRequest, GetOrderResponse, GetStatsRequest, GetStatsResponse, Order, OrderSide,
    OrderStatus, OrderType,
};
use super::server::CustomOrderbook;
use super::server::OrderbookGrpcService;
use lobster::{
    OrderEvent as LobsterOrderEvent, OrderType as LobsterOrderType, Side as LobsterOrderSide,
};
use tonic::Request;
use uuid::Uuid;

impl From<LobsterOrderSide> for OrderSide {
    fn from(value: LobsterOrderSide) -> Self {
        match value {
            LobsterOrderSide::Ask => OrderSide::Ask,
            LobsterOrderSide::Bid => OrderSide::Bid,
        }
    }
}

impl From<OrderSide> for LobsterOrderSide {
    fn from(value: OrderSide) -> Self {
        match value {
            OrderSide::Ask => LobsterOrderSide::Ask,
            OrderSide::Bid => LobsterOrderSide::Bid,
        }
    }
}

/// add order
pub(crate) fn add_order(
    grpc: &OrderbookGrpcService,
    request: Request<AddOrderRequest>,
) -> Result<AddOrderResponse, GrpcError> {
    let request = request.into_inner();
    let order_id = Uuid::new_v4().as_u128();
    let qty = request
        .quantity
        .parse::<u64>()
        .map_err(|e| GrpcError::Orderbook(OrderbookError::UnparsableOrderQuantity(e)))?;
    let order_side = OrderSide::from_i32(request.order_side)
        .ok_or_else(|| GrpcError::Orderbook(OrderbookError::UnparsableOrderSide))?;
    let price = request.price.parse::<u64>();

    let order = match OrderType::from_i32(request.order_type) {
        Some(OrderType::Limit) => {
            let price = price
                .clone()
                .map_err(|e| GrpcError::Orderbook(OrderbookError::UnparsableOrderPrice(e)))?;
            LobsterOrderType::Limit {
                id: order_id,
                qty,
                price,
                side: order_side.into(),
            }
        }
        Some(OrderType::Market) => LobsterOrderType::Market {
            id: order_id,
            qty,
            side: order_side.into(),
        },
        _ => return Err(GrpcError::Orderbook(OrderbookError::UnknownOrderType)),
    };

    let mut guard = grpc
        .orderbook_client
        .lock()
        .map_err(|_| GrpcError::Orderbook(OrderbookError::LockError))?;

    let (ticker, event) = match guard.get_mut(&request.orderbook_id) {
        Some(ticker_obook) => {
            let event = ticker_obook.get_orderbook_mut().execute(order);
            let ticker = ticker_obook.get_ticker();
            (ticker, event)
        }
        None => {
            return Err(GrpcError::Orderbook(
                OrderbookError::NonExistentOrderbookUuid,
            ));
        }
    };

    let order = match event {
        LobsterOrderEvent::Canceled { id } => Order {
            orderbook_id: request.orderbook_id.to_string(),
            id: id.to_string(),
            ticker: ticker.to_string(),
            price: price.ok().unwrap_or_default().to_string(),
            quantity: qty.to_string(),
            order_side: order_side.into(),
            order_type: request.order_type.into(),
            status: OrderStatus::Cancelled.into(),
            partial_fills: vec![],
        },
        LobsterOrderEvent::Filled {
            id,
            filled_qty,
            fills,
        } => Order {
            orderbook_id: request.orderbook_id.to_string(),
            id: id.to_string(),
            ticker: ticker.to_string(),
            price: price.ok().unwrap_or_default().to_string(),
            quantity: filled_qty.to_string(),
            order_side: order_side.into(),
            order_type: request.order_type.into(),
            status: OrderStatus::Filled.into(),
            partial_fills: fills
                .iter()
                .map(|fill| FillMetadata {
                    order_1: fill.order_1.to_string(),
                    order_2: fill.order_2.to_string(),
                    price: fill.price.to_string(),
                    qty: fill.qty.to_string(),
                    taker_side: OrderSide::from(fill.taker_side).into(),
                    total_fill: fill.total_fill,
                })
                .collect::<Vec<FillMetadata>>(),
        },
        LobsterOrderEvent::PartiallyFilled {
            id,
            filled_qty,
            fills,
        } => Order {
            orderbook_id: request.orderbook_id.to_string(),
            id: id.to_string(),
            ticker: ticker.to_string(),
            price: price.ok().unwrap_or_default().to_string(),
            quantity: filled_qty.to_string(),
            order_side: order_side.into(),
            order_type: request.order_type.into(),
            status: OrderStatus::PartiallyFilled.into(),
            partial_fills: fills
                .iter()
                .map(|fill| FillMetadata {
                    order_1: fill.order_1.to_string(),
                    order_2: fill.order_2.to_string(),
                    price: fill.price.to_string(),
                    qty: fill.qty.to_string(),
                    taker_side: OrderSide::from(fill.taker_side).into(),
                    total_fill: fill.total_fill,
                })
                .collect::<Vec<FillMetadata>>(),
        },
        LobsterOrderEvent::Placed { id } => Order {
            orderbook_id: request.orderbook_id.to_string(),
            id: id.to_string(),
            ticker: ticker.to_string(),
            price: price.ok().unwrap_or_default().to_string(),
            quantity: qty.to_string(),
            order_side: order_side.into(),
            order_type: request.order_type.into(),
            status: OrderStatus::Placed.into(),
            partial_fills: vec![],
        },
        LobsterOrderEvent::Unfilled { id } => Order {
            orderbook_id: request.orderbook_id.to_string(),
            id: id.to_string(),
            ticker: ticker.to_string(),
            price: price.ok().unwrap_or_default().to_string(),
            quantity: qty.to_string(),
            order_side: order_side.into(),
            order_type: request.order_type.into(),
            status: OrderStatus::Unfilled.into(),
            partial_fills: vec![],
        },
    };

    let sender = grpc.resources_ctx.clone();

    // TODO: fix this code!
    // send result to rabbitmq
    /*
    tokio::spawn(async move {
        let confirm = sender
            .amqp_pool_manager
            .publish_msg("", "hello", b"MESSAGE")
            .await
            .map_err(|e| GrpcError::Amqp(e))
            .unwrap();
        let confirm = confirm
            .await
            .map_err(|e| GrpcError::Amqp(AmqpError::RMQError(e)))
            .unwrap();
        tracing::info!("Amqp confirmation received: {:?}", confirm);
    });
    */

    // persist trade to DB
    /*
    let p = sqlx::query(
        "INSERT INTO questions (orderbook_id,
            created_at,
            ticker,
            order_type,
            order_side,
            order_status,
            order_value,
            order_quantity,
            logs)
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9)
        RETURNING id, orderbook_id, created_at, ticker, order_type, order_side, order_status, order_value, order_quantity, logs"
        )
        .bind(orderbook_id)
        .bind(new_question.content)
        .bind(new_question.tags);
    */

    Ok(AddOrderResponse { order: Some(order) })
}

pub(crate) fn cancel_order(
    grpc: &OrderbookGrpcService,
    request: Request<CancelOrderRequest>,
) -> Result<CancelOrderResponse, GrpcError> {
    let request = request.into_inner();

    let mut guard = grpc
        .orderbook_client
        .lock()
        .map_err(|_| GrpcError::Orderbook(OrderbookError::LockError))?;

    let ticker_obook = guard.get_mut(&request.orderbook_id);

    let event = match ticker_obook {
        Some(ticker_obook) => {
            let order_id = request
                .order_id
                .parse::<u128>()
                .map_err(|e| GrpcError::Orderbook(OrderbookError::UnparsableOrderId(e)))?;
            let order = LobsterOrderType::Cancel { id: order_id };
            let event = ticker_obook.get_orderbook_mut().execute(order);
            drop(guard);
            event
        }
        None => {
            drop(guard);
            return Err(GrpcError::Orderbook(
                OrderbookError::NonExistentOrderbookUuid,
            ));
        }
    };

    match event {
        LobsterOrderEvent::Canceled { id } => {
            tracing::info!("Canceled id {:?}", id);
        }
        _ => return Err(GrpcError::Orderbook(OrderbookError::WrongOrderIdCancel)),
    }

    Ok(CancelOrderResponse {
        status: OrderStatus::Cancelled.into(),
    })
}

pub(crate) fn get_order(
    grpc: &OrderbookGrpcService,
    request: Request<GetOrderRequest>,
) -> Result<GetOrderResponse, GrpcError> {
    let request = request.into_inner();

    let guard = grpc
        .orderbook_client
        .lock()
        .map_err(|_| GrpcError::Orderbook(OrderbookError::LockError))?;

    let order_details = match guard.get(&request.orderbook_id) {
        Some(custom_orderbook) => {
            let order_id = request
                .order_id
                .parse::<u128>()
                .map_err(|e| GrpcError::Orderbook(OrderbookError::UnparsableOrderId(e)))?;
            let order_details = custom_orderbook
                .get_orderbook()
                .order_arena()
                .get_full(order_id)
                .ok_or_else(|| GrpcError::Orderbook(OrderbookError::NonExistentOrderId))?;
            let ticker = custom_orderbook.get_ticker();
            (ticker, order_details.0, order_details.1)
        }
        None => {
            return Err(GrpcError::Orderbook(
                OrderbookError::NonExistentOrderbookUuid,
            ));
        }
    };

    Ok(GetOrderResponse {
        order: Some(Order {
            orderbook_id: request.orderbook_id,
            id: request.order_id,
            ticker: order_details.0.to_string(),
            price: order_details.1.to_string(),
            quantity: order_details.2.to_string(),
            status: OrderStatus::Placed.into(),
            order_type: OrderType::Market.into(),
            partial_fills: vec![],
            ..Default::default()
        }),
    })
}

pub(crate) fn get_stats(
    grpc: &OrderbookGrpcService,
    request: Request<GetStatsRequest>,
) -> Result<GetStatsResponse, GrpcError> {
    let request = request.into_inner();

    let guard = grpc
        .orderbook_client
        .lock()
        .map_err(|_| GrpcError::Orderbook(OrderbookError::LockError))?;

    let ticker_obook = guard.get(&request.orderbook_id);

    let (max_bid, max_ask, spread, traded_volume) = match ticker_obook {
        Some(ticker_obook) => {
            let max_bid = ticker_obook.get_orderbook().max_bid();
            let max_ask = ticker_obook.get_orderbook().min_ask();
            let spread = ticker_obook.get_orderbook().spread();
            let traded_volume = ticker_obook.get_orderbook().traded_volume().to_string();
            drop(guard);
            (max_bid, max_ask, spread, traded_volume)
        }
        None => {
            drop(guard);
            return Err(GrpcError::Orderbook(
                OrderbookError::NonExistentOrderbookUuid,
            ));
        }
    };

    Ok(GetStatsResponse {
        max_ask: max_ask.unwrap_or_default().to_string(),
        max_bid: max_bid.unwrap_or_default().to_string(),
        spread: spread.unwrap_or_default().to_string(),
        traded_volume,
    })
}

pub(crate) fn create_orderbook(
    grpc: &OrderbookGrpcService,
    request: Request<CreateOrderbookRequest>,
) -> Result<CreateOrderbookResponse, GrpcError> {
    let request = request.into_inner();

    let mut guard = grpc
        .orderbook_client
        .lock()
        .map_err(|_| GrpcError::Orderbook(OrderbookError::LockError))?;

    let orderbook_uuid = Uuid::new_v4();
    guard.insert(
        orderbook_uuid.to_string(),
        CustomOrderbook::new(
            orderbook_uuid.to_string(),
            grpc.orderbook_config.clone(),
            request.ticker,
            None,
            request.name,
        ),
    );
    drop(guard);

    Ok(CreateOrderbookResponse {
        uuid: orderbook_uuid.to_string(),
    })
}

pub(crate) fn delete_orderbook(
    grpc: &OrderbookGrpcService,
    request: Request<DeleteOrderbookRequest>,
) -> Result<DeleteOrderbookResponse, GrpcError> {
    let request = request.into_inner();

    let mut guard = grpc
        .orderbook_client
        .lock()
        .map_err(|_| GrpcError::Orderbook(OrderbookError::LockError))?;

    let deleted_orderbook = guard.remove(&request.uuid);
    drop(guard);

    Ok(DeleteOrderbookResponse {
        success: deleted_orderbook.is_some(),
    })
}
