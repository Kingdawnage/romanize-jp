FROM rust:slim

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN cargo build --release

EXPOSE 5000

CMD cargo run --release