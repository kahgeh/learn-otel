ARG BASEPATH=/app
ARG COMPONENT="client"
FROM rust:1.62.0 as planner
ARG BASEPATH
WORKDIR ${BASEPATH}
# We only pay the installation cost once,
# it will be cached from the second build onwards
RUN cargo install cargo-chef
RUN rustup component add rustfmt
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.62.0 as cacher
ARG BASEPATH
WORKDIR ${BASEPATH}
RUN cargo install cargo-chef
RUN rustup component add rustfmt
COPY --from=planner ${BASEPATH}/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.62.0 as builder
ARG BASEPATH
ARG COMPONENT
WORKDIR ${BASEPATH}
RUN rustup component add rustfmt
COPY . .
# Copy over the cached dependencies
COPY --from=cacher ${BASEPATH}/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cd client && cargo build --release --bin ${COMPONENT}

FROM rust:1.62.0 as runtime
ARG BASEPATH
ARG COMPONENT
WORKDIR ${BASEPATH}
EXPOSE 6000
COPY --from=builder ${BASEPATH}/target/release/${COMPONENT} .
RUN chmod +x ./${COMPONENT}
RUN mkdir -p config/${COMPONENT}

CMD ["./client"]
