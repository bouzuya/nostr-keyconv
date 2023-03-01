FROM rust:1.67-alpine AS builder
WORKDIR /home/rust
RUN apk update && apk add --no-cache musl-dev
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN mkdir src/
RUN touch src/lib.rs
RUN cargo build --release --target=x86_64-unknown-linux-musl
COPY . .
RUN cargo install --path . --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /usr/local/cargo/bin/nostr-keyconv /usr/local/bin/nostr-keyconv
ENTRYPOINT ["nostr-keyconv"]
