# (1) installing cargo-chef & build deps
FROM rust:alpine AS chef
WORKDIR /app
RUN apk add --no-cache musl-dev openssl-dev
RUN cargo install --locked cargo-chef

# (2) preparing recipe file
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# (3) building project deps, cache magic happen on COPY command
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json --release

# (4) actual project build
COPY . .
RUN cargo build -r

# (5) runtime image, you can use any base image you want
FROM scratch AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/fiars /app/fiars
CMD ["/app/fiars"]