use std::{error::Error, net::SocketAddr};

use axum::{response::Json, routing::get, Router};
use common::{logging, process_control::shutdown_signal};
use hyper::HeaderMap;

use opentelemetry::global;
use opentelemetry_http::HeaderInjector;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, span, Level, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::setup("learn-otel-client", "bla bla")?;
    let app: Router;
    let addr: SocketAddr;
    {
        let config_span = span!(Level::INFO, "configuration");
        let _config_entered = config_span.enter();
        info!(r#"setting routes and port {}"#, port = 6000);
        app = Router::new().route("/", get(root));
        addr = SocketAddr::from(([0, 0, 0, 0], 6000));
    }

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    {
        let teardown_span = span!(Level::INFO, "teardown");
        let _teardown_entered = teardown_span.enter();
        info!("teardown complete");
    }

    logging::teardown();

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct HelloResponse {
    message: String,
}

#[derive(Serialize, Deserialize)]
struct SubjectResponse {
    subject: String,
}

#[instrument(level = "info")]
async fn root() -> Json<HelloResponse> {
    info!("forming hello...");
    let mut request = reqwest::get("http://server:6001").await.unwrap();
    global::get_text_map_propagator(|propagator| {
        let cx = Span::current().context();
        println!("{:?} {:?}", Span::current(), cx);
        propagator.inject_context(&cx, &mut HeaderInjector(request.headers_mut()))
    });
    println!("{:?}", request.headers());
    let subject_response = request.json::<SubjectResponse>().await.unwrap();

    Json(HelloResponse {
        message: String::from(format!("Hello {}", subject_response.subject)),
    })
}
