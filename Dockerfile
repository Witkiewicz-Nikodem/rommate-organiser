FROM rust AS builder
WORKDIR /rust-api-deployment-example
#copy the source code
COPY . .
# Build th appliaction
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release 
RUN ls -lR target

# Create a new stage with a minimal image
# FROM debian:bookworm
# COPY --from=builder /rust-api-deployment-example/target/release/DB_DIESEL_TYPES /api-deployment-example
ENTRYPOINT ["/rust-api-deployment-example/target/release/DB_DIESEL_TYPES"]
EXPOSE 3000

