FROM rust:1.83

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN cargo build --release
RUN cp ./target/release/andromeda-galaxy /bin/server

ENV SSL_CERT_FILE=/dev/null
EXPOSE 8080

ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/bin/server"]
