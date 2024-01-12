# BUILD FRONT
FROM node:21-alpine as builder-front
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY frontend .
RUN npm run build

# BUILD BACK
FROM rust:latest as builder-back
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
# download and build dependencies into cache
COPY Cargo.* ./
RUN \
    mkdir src && \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -rf src
# build our actual code
COPY src src
RUN \
    touch src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl

# START BACK WITH FRONT
FROM alpine:latest
COPY --from=builder-back /app/target/x86_64-unknown-linux-musl/release/togetherness /srv/togetherness
COPY --from=builder-front /app/build /srv/front
CMD ["/srv/togetherness"]
