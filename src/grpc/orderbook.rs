#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FillMetadata {
    /// / The ID of the order that triggered the fill (taker).
    #[prost(string, tag = "1")]
    pub order_1: ::prost::alloc::string::String,
    /// / The ID of the matching order.
    #[prost(string, tag = "2")]
    pub order_2: ::prost::alloc::string::String,
    /// / The quantity that was traded.
    #[prost(string, tag = "3")]
    pub qty: ::prost::alloc::string::String,
    /// / The price at which the trade happened.
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
    /// / The side of the taker order (order 1)
    #[prost(enumeration = "OrderSide", tag = "5")]
    pub taker_side: i32,
    /// / Whether this order was a total (true) or partial (false) fill of the
    /// / maker order.
    #[prost(bool, tag = "6")]
    pub total_fill: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    #[prost(string, tag = "1")]
    pub orderbook_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderType", tag = "4")]
    pub order_type: i32,
    #[prost(enumeration = "OrderSide", tag = "5")]
    pub order_side: i32,
    #[prost(string, tag = "6")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderStatus", tag = "8")]
    pub status: i32,
    #[prost(message, repeated, tag = "9")]
    pub partial_fills: ::prost::alloc::vec::Vec<FillMetadata>,
}
/// add order
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrderRequest {
    #[prost(string, tag = "1")]
    pub orderbook_id: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderType", tag = "2")]
    pub order_type: i32,
    #[prost(enumeration = "OrderSide", tag = "3")]
    pub order_side: i32,
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub quantity: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// cancel order
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderRequest {
    #[prost(string, tag = "1")]
    pub orderbook_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderResponse {
    #[prost(enumeration = "OrderStatus", tag = "1")]
    pub status: i32,
}
/// get order status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderRequest {
    #[prost(string, tag = "1")]
    pub orderbook_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderResponse {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// get stats request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatsRequest {
    #[prost(string, tag = "1")]
    pub orderbook_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatsResponse {
    #[prost(string, tag = "1")]
    pub max_bid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub max_ask: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub spread: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub traded_volume: ::prost::alloc::string::String,
}
/// CreateOrderbookRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrderbookRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrderbookResponse {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
}
/// Delete Orderbook Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrderbookRequest {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrderbookResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// enums
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide {
    /// / The ask (or sell) side.
    Ask = 0,
    /// / The bid (or buy) side.
    Bid = 1,
}
impl OrderSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderSide::Ask => "Ask",
            OrderSide::Bid => "Bid",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Ask" => Some(Self::Ask),
            "Bid" => Some(Self::Bid),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    /// / A limit order, which is either filled immediately, or added to the order
    /// / book.
    Limit = 0,
    /// / A market order, which is either filled immediately (even partially), or
    /// / canceled.
    Market = 1,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Limit => "Limit",
            OrderType::Market => "Market",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Limit" => Some(Self::Limit),
            "Market" => Some(Self::Market),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderStatus {
    /// / Indicating that the corresponding order was not filled. It is only sent
    /// / in response to market orders.
    Unfilled = 0,
    /// / Indicating that the corresponding order was placed on the order book. It
    /// / is only send in response to limit orders.
    Placed = 1,
    /// / Indicating that the corresponding order was removed from the order book.
    /// / It is only sent in response to cancel orders.
    Cancelled = 2,
    /// / Indicating that the corresponding order was only partially filled. It is
    /// / sent in response to market or limit orders.
    PartiallyFilled = 3,
    /// / Indicating that the corresponding order was filled completely. It is
    /// / sent in response to market or limit orders.
    Filled = 4,
}
impl OrderStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderStatus::Unfilled => "Unfilled",
            OrderStatus::Placed => "Placed",
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::PartiallyFilled => "PartiallyFilled",
            OrderStatus::Filled => "Filled",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unfilled" => Some(Self::Unfilled),
            "Placed" => Some(Self::Placed),
            "Cancelled" => Some(Self::Cancelled),
            "PartiallyFilled" => Some(Self::PartiallyFilled),
            "Filled" => Some(Self::Filled),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod orderbook_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service Definition
    #[derive(Debug, Clone)]
    pub struct OrderbookClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrderbookClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrderbookClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrderbookClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OrderbookClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// / create and delete orderbooks
        pub async fn create_orderbook(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrderbookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orderbook.Orderbook/createOrderbook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("orderbook.Orderbook", "createOrderbook"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_orderbook(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrderbookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orderbook.Orderbook/deleteOrderbook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("orderbook.Orderbook", "deleteOrderbook"));
            self.inner.unary(req, path, codec).await
        }
        /// / add an order, returning immediately an event indicating the result.
        pub async fn add_order(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orderbook.Orderbook/addOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("orderbook.Orderbook", "addOrder"));
            self.inner.unary(req, path, codec).await
        }
        /// / cancel order, which removes the order with the specified ID from the order book.
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orderbook.Orderbook/cancelOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("orderbook.Orderbook", "cancelOrder"));
            self.inner.unary(req, path, codec).await
        }
        /// / returns the order data and status
        pub async fn get_order(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orderbook.Orderbook/getOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("orderbook.Orderbook", "getOrder"));
            self.inner.unary(req, path, codec).await
        }
        /// / returns the stats
        pub async fn get_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStatsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/orderbook.Orderbook/getStats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("orderbook.Orderbook", "getStats"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod orderbook_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrderbookServer.
    #[async_trait]
    pub trait Orderbook: Send + Sync + 'static {
        /// / create and delete orderbooks
        async fn create_orderbook(
            &self,
            request: tonic::Request<super::CreateOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrderbookResponse>,
            tonic::Status,
        >;
        async fn delete_orderbook(
            &self,
            request: tonic::Request<super::DeleteOrderbookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrderbookResponse>,
            tonic::Status,
        >;
        /// / add an order, returning immediately an event indicating the result.
        async fn add_order(
            &self,
            request: tonic::Request<super::AddOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddOrderResponse>,
            tonic::Status,
        >;
        /// / cancel order, which removes the order with the specified ID from the order book.
        async fn cancel_order(
            &self,
            request: tonic::Request<super::CancelOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelOrderResponse>,
            tonic::Status,
        >;
        /// / returns the order data and status
        async fn get_order(
            &self,
            request: tonic::Request<super::GetOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderResponse>,
            tonic::Status,
        >;
        /// / returns the stats
        async fn get_stats(
            &self,
            request: tonic::Request<super::GetStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStatsResponse>,
            tonic::Status,
        >;
    }
    /// Service Definition
    #[derive(Debug)]
    pub struct OrderbookServer<T: Orderbook> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Orderbook> OrderbookServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrderbookServer<T>
    where
        T: Orderbook,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/orderbook.Orderbook/createOrderbook" => {
                    #[allow(non_camel_case_types)]
                    struct createOrderbookSvc<T: Orderbook>(pub Arc<T>);
                    impl<
                        T: Orderbook,
                    > tonic::server::UnaryService<super::CreateOrderbookRequest>
                    for createOrderbookSvc<T> {
                        type Response = super::CreateOrderbookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrderbookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_orderbook(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createOrderbookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orderbook.Orderbook/deleteOrderbook" => {
                    #[allow(non_camel_case_types)]
                    struct deleteOrderbookSvc<T: Orderbook>(pub Arc<T>);
                    impl<
                        T: Orderbook,
                    > tonic::server::UnaryService<super::DeleteOrderbookRequest>
                    for deleteOrderbookSvc<T> {
                        type Response = super::DeleteOrderbookResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrderbookRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_orderbook(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteOrderbookSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orderbook.Orderbook/addOrder" => {
                    #[allow(non_camel_case_types)]
                    struct addOrderSvc<T: Orderbook>(pub Arc<T>);
                    impl<
                        T: Orderbook,
                    > tonic::server::UnaryService<super::AddOrderRequest>
                    for addOrderSvc<T> {
                        type Response = super::AddOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).add_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = addOrderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orderbook.Orderbook/cancelOrder" => {
                    #[allow(non_camel_case_types)]
                    struct cancelOrderSvc<T: Orderbook>(pub Arc<T>);
                    impl<
                        T: Orderbook,
                    > tonic::server::UnaryService<super::CancelOrderRequest>
                    for cancelOrderSvc<T> {
                        type Response = super::CancelOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).cancel_order(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = cancelOrderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orderbook.Orderbook/getOrder" => {
                    #[allow(non_camel_case_types)]
                    struct getOrderSvc<T: Orderbook>(pub Arc<T>);
                    impl<
                        T: Orderbook,
                    > tonic::server::UnaryService<super::GetOrderRequest>
                    for getOrderSvc<T> {
                        type Response = super::GetOrderResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrderRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_order(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getOrderSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/orderbook.Orderbook/getStats" => {
                    #[allow(non_camel_case_types)]
                    struct getStatsSvc<T: Orderbook>(pub Arc<T>);
                    impl<
                        T: Orderbook,
                    > tonic::server::UnaryService<super::GetStatsRequest>
                    for getStatsSvc<T> {
                        type Response = super::GetStatsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStatsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_stats(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getStatsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Orderbook> Clone for OrderbookServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Orderbook> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Orderbook> tonic::server::NamedService for OrderbookServer<T> {
        const NAME: &'static str = "orderbook.Orderbook";
    }
}
