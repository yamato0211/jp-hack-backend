FROM rust:1.64 
WORKDIR /app
RUN cargo install cargo-watch
COPY . .