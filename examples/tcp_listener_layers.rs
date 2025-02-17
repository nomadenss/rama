use rama::{
    service::{layer::TimeoutLayer, ServiceBuilder},
    stream::service::EchoService,
    tcp::server::TcpListener,
};
use std::time::Duration;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env_lossy(),
        )
        .init();

    let graceful = rama::graceful::Shutdown::default();

    graceful.spawn_task_fn(|guard| async {
        TcpListener::bind("127.0.0.1:9000")
            .await
            .expect("bind TCP Listener")
            .serve_graceful(
                guard,
                ServiceBuilder::new()
                    .trace_err()
                    .layer(TimeoutLayer::new(Duration::from_secs(8)))
                    .service(EchoService::new()),
            )
            .await;
    });

    graceful
        .shutdown_with_limit(Duration::from_secs(30))
        .await
        .expect("graceful shutdown");
}
