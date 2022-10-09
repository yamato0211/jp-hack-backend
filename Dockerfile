FROM rust:1.64 
WORKDIR /app
RUN apt update
RUN apt install -y libpq-dev build-essential
RUN cargo install cargo-watch
COPY . .