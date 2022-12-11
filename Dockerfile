# Build the image using pinned nightly (same as in shell.nix)
FROM clux/muslrust:nightly-2022-12-11 as builder
WORKDIR /app

# Cache dependencies
RUN cargo init
ADD Cargo.toml Cargo.lock ./
RUN cargo run --release

# Now remove hello world source
RUN rm -rf src
# Remove fingerprint
RUN find target -name 'temporal*' -type d -exec rm -rf {} \; || true
# Remove binary
RUN find target -name 'temporal' -type f -exec rm -rf {} \; || true

# Actually build the project
ADD . ./
# The SQLX_OFFLINE ensures it uses sqlx-data.json for checking the types,
# instead of trying to connect to a running database.
RUN SQLX_OFFLINE=true cargo build --release

# Runtime image
FROM alpine as runtime
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/temporal ./

# Make sure postgres (in sqlx) can use tls
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
CMD ./temporal