FROM rust:1.58

WORKDIR /usr/src/backend-rust
#COPY . .

#RUN cargo install --path .
# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

CMD ["backend-rust"]