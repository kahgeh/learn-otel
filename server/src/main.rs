use std::{error::Error, net::SocketAddr};

use axum::{body::Body, http::Request, response::Json, routing::get, Router};
use common::{logging, process_control::shutdown_signal};
use opentelemetry::global;
use opentelemetry_http::HeaderExtractor;
use serde::{Deserialize, Serialize};
use tracing::{info, instrument, span, Level, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logging::setup("learn-otel-server", "bla bla")?;
    let app: Router;
    let addr: SocketAddr;
    {
        let config_span = span!(Level::INFO, "configuration");
        let _config_entered = config_span.enter();
        info!(r#"setting routes and port {}"#, port = 6001);
        app = Router::new().route("/", get(root));
        addr = SocketAddr::from(([0, 0, 0, 0], 6001));
    }
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    {
        let teardown_span = span!(Level::INFO, "teardown");
        let _teardown_entered = teardown_span.enter();
        info!("teardown complete");
    }

    logging::teardown();
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct SubjectResponse {
    subject: String,
}

#[instrument(level = "info")]
async fn root(request: Request<Body>) -> Json<SubjectResponse> {
    println!("{:?}", request.headers());
    let parent_cx = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });
    Span::current().set_parent(parent_cx.clone());
    info!("returning subject...");
    Json(SubjectResponse {
        subject: String::from("world"),
    })
}
