use anyhow::{Context, Result};
use argh::{self, FromArgs};
use orderbook_api::config::Config;
use orderbook_api::context::Context as ResourcesContext;
use orderbook_api::error::handle_rejection;
use orderbook_api::grpc::server::{OrderbookGrpcService, StopHandle};
use orderbook_api::http::filters::with_cors;
use orderbook_api::http::routes::{healthcheck_route, homepage_route};
use orderbook_api::sqlx::pool::DbConnector;
use std::sync::Arc;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::broadcast;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    // init config
    dotenv::dotenv().ok();
    let args: Args = argh::from_env();
    println!("{:?}", args.config);

    // set up log filter to be used by tracing
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "orderbook_rs=info,warp=error".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // init config
    let config = Config::new(args.config)
        .await
        .context("Failed to load config")?;

    let db_client = DbConnector::new(&config.db).await?;
    tracing::info!(
        "Connected to postgres database running on {:?}:{:?}",
        &config.db.host,
        &config.db.port
    );
    let resources_ctx = Arc::new(ResourcesContext { db_client });

    // http routes
    let homepage_route = homepage_route();
    let healthcheck_route = healthcheck_route(resources_ctx.clone());

    // bundle routes
    let routes = homepage_route
        .or(healthcheck_route)
        .with(with_cors())
        .recover(handle_rejection);

    // setup the grpc server
    let orderbook_service =
        OrderbookGrpcService::new(config.orderbook.clone(), resources_ctx.clone());

    // stop signal for http
    let (http_stop_tx, mut http_stop_rx) = broadcast::channel(1);

    // setup the http server
    let (_addr, http_server) =
        warp::serve(routes).bind_with_graceful_shutdown(config.api.http.bind, async move {
            tracing::info!(
                "Http server: listening on {}",
                config.api.http.bind.to_string()
            );
            tracing::info!("Http Server: waiting for a shutdown signal...");
            _ = http_stop_rx.recv().await;

            tracing::info!("Http Server: cleaning up resources..."); //TODO ???
            tracing::info!("Http Server: done cleaning resources!");
            tracing::info!("Http Server: Exiting cleanly ...!")
        });

    // run grpc server in the background
    let grpc_stop_tx = match orderbook_service.serve_async(config.api.grpc.clone()).await {
        Ok(s) => {
            tracing::info!(
                "Grpc server: started successfully on {:?}",
                config.api.grpc.bind.to_string()
            );
            tracing::info!("Grpc server: waiting for a shutdown signal...");
            Some(s)
        }
        Err(err) => {
            tracing::error!("Grpc server: Join Error {}", err.to_string());
            None
        }
    };

    // spawn terminate handlers routine
    tokio::spawn(stop_signal(http_stop_tx, grpc_stop_tx));

    // run http server
    let _ = tokio::join!(tokio::task::spawn(http_server));

    Ok(())
}

async fn stop_signal(http_stop_tx: broadcast::Sender<()>, grpc_stop_tx: Option<StopHandle>) {
    let mut sigint = signal(SignalKind::interrupt()).expect("shutdown_listener");
    let mut sigterm = signal(SignalKind::terminate()).expect("shutdown_listener");
    tokio::select! {
        _ = sigint.recv() => {
            tracing::info!("Received SIGINT ...");
            let _ = http_stop_tx.send(());
            if let Some(grpc_handle) = grpc_stop_tx {
                grpc_handle.stop()
            }
        }
        _ = sigterm.recv() => {
            tracing::info!("Received SIGTERM ...");
            let _ = http_stop_tx.send(());
            if let Some(grpc_handle) = grpc_stop_tx {
                grpc_handle.stop()
            }
        }
    }
}

/// Orderbook Service
#[derive(FromArgs)]
struct Args {
    /// path to the config file
    #[argh(option, short = 'c')]
    config: String,
}
