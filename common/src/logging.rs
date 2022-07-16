use opentelemetry::{
    global,
    sdk::{export::trace::stdout, propagation::TraceContextPropagator, trace, Resource},
    KeyValue,
};

use std::{error::Error, process};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

pub fn setup(service_name: &str, service_version: &str) -> Result<(), Box<dyn Error>> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .with_trace_config(
            trace::config()
                .with_sampler(opentelemetry::sdk::trace::Sampler::AlwaysOn)
                .with_resource(Resource::new(vec![
                    KeyValue::new("service.version", String::from(service_version)),
                    KeyValue::new("process.pid", process::id().to_string()),
                ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    let otel = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .with(otel)
        .init();

    Ok(())
}

pub fn simple_setup(service_name: &str, service_version: &str) -> Result<(), Box<dyn Error>> {
    let tracer = stdout::new_pipeline()
        .with_trace_config(trace::config().with_resource(Resource::new(vec![
            KeyValue::new("service.name", String::from(service_name)),
            KeyValue::new("service.version", String::from(service_version)),
        ])))
        .with_pretty_print(true)
        .install_simple();
    let otel = tracing_opentelemetry::layer().with_tracer(tracer);
    Registry::default().with(otel).init();

    Ok(())
}

pub fn teardown() {
    opentelemetry::global::shutdown_tracer_provider();
}
