// SPDX-FileCopyrightText: 2022 jerusdp
//
// SPDX-License-Identifier: MIT OR Apache-2.0

mod tc001_blank_sitekey;
use tc001_blank_sitekey::tc001;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn root() -> &'static str {
    "Hello, World!"
}

async fn siteverify() -> &'static str {
    r#"tc001 - Blank Sitekey"#
}

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/", get(root))
//         .route("/siteverify", get(siteverify))
//         .route("/siteverify/tc001", post(tc001));

//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app()).await;
}

/// Having a function that produces our app makes it easy to call it from tests
/// without having to create an HTTP server.
fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/siteverify", get(siteverify))
        .route("/siteverify/tc001", post(tc001))
        .layer(TraceLayer::new_for_http())
}

#[cfg(test)]
mod test {
    use crate::tc001_blank_sitekey::{RequestData, SuccessResponse};
    use axum_test::TestServer;

    fn test_server() -> TestServer {
        TestServer::new(super::app())
    }

    #[tokio::test]
    async fn hello_world() {
        let server = test_server();

        let response = server.get("/").await;

        response.assert_text("Hello, World!");
    }

    #[tokio::test]
    async fn tc001() {
        let server = test_server();

        let request = RequestData {
            response: Some("10000000-aaaa-bbbb-cccc-000000000001".to_string()),
            secret: Some("0x0000000000000000000000000000000000000000".to_string()),
            sitekey: Some("10000000-ffff-ffff-ffff-000000000001".to_string()),
            ..Default::default()
        };

        let response = server.post("/siteverify/tc001").form(&request).await;

        response.assert_status_ok();

        let report = response.form::<SuccessResponse>();
        assert!(report.success());
        assert!(!report.credit());
        assert_eq!(report.hostname(), "dummy-key-pass");
    }
}
