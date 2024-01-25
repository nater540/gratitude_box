#############################################
# Base image
#############################################
FROM rust:1.75-alpine3.19 as base
WORKDIR /usr/src/app

RUN apk add --no-cache build-base libc-dev openssl-dev postgresql-dev

#############################################
# Builder image
#############################################
FROM base as builder

COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release

RUN rm -rf ./src
COPY ./src ./src

RUN touch ./src/main.rs
RUN cargo build --release

#############################################
# Run image
#############################################
FROM alpine:3.19 as run
RUN apk add --no-cache ca-certificates openssl
COPY --from=builder /usr/src/app/target/release/gratitude_box /usr/local/bin
ENTRYPOINT ["/usr/local/bin/gratitude_box"]
