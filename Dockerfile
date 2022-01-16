FROM rust:1.58

WORKDIR /usr/src/backend-rust
COPY . .

RUN cargo install --path .

CMD ["backend-rust"]