FROM rust:1

WORKDIR /usr/src/accorciami

COPY . .

RUN cargo build --release
RUN cargo install --path .

CMD ["/usr/local/cargo/bin/accorciami"]
