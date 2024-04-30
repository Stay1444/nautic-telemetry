FROM rust AS rust-builder

RUN mkdir /app
WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian

RUN apt-get update && apt-get install -y openssl
RUN mkdir /app

COPY --from=rust-builder /app/target/release/influx-feeder /usr/bin/influx-feeder

RUN chmod +x /usr/bin/influx-feeder
RUN apt-get update && apt-get install -y libssl-dev

WORKDIR /app

ENTRYPOINT [ "/usr/bin/influx-feeder" ]
