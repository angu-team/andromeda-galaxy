FROM rust:1.83

WORKDIR /app
COPY . .

RUN cargo build --release
RUN cp ./target/release/andromeda-galaxy /bin/server

ENV ROCKET_ADDRESS=0.0.0.0
CMD ["/bin/server"]
