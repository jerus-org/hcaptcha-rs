use lambda_runtime::Error;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

mod handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    LogTracer::init()?;

    let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "INFO".to_owned()),
        ))
        .with(JsonStorageLayer)
        .with(bunyan_formatting_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    lambda_runtime::run(lambda_runtime::service_fn(handler::my_handler)).await?;

    Ok(())
}
