use opentelemetry::{
    sdk::{export::trace::stdout, trace, Resource},
    KeyValue,
};

use std::{error::Error, thread, time::Duration};
use tracing::span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Registry};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let tracer = stdout::new_pipeline()
        .with_trace_config(trace::config().with_resource(Resource::new(vec![
            KeyValue::new("service.name", "learn-otel-app"),
            KeyValue::new("service.version", "bla bla"),
        ])))
        .install_simple();
    let otel = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default().with(otel).init();
    {
        let root = span!(tracing::Level::INFO, "app_start",);
        let _enter = root.enter();

        span!(tracing::Level::INFO, "slow work")
            .in_scope(|| thread::sleep(Duration::from_millis(25)));

        span!(tracing::Level::INFO, "faster work")
            .in_scope(|| thread::sleep(Duration::from_millis(10)));
    }

    Ok(())
}
