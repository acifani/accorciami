FROM rust:1 as builder

WORKDIR /usr/src/accorciami

COPY . .
RUN cargo build --release

FROM debian:11-slim

WORKDIR /home/accorciami/bin
COPY --from=builder /usr/src/accorciami/target/release/accorciami .

CMD ["./accorciami"]
