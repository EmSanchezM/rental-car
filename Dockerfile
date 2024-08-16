FROM rust:latest

RUN apt-get update && apt-get install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

CMD ["cargo", "run", "--release"]
