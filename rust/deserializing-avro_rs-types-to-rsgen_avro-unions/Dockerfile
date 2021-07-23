FROM rust:1.53.0-slim-buster AS build

WORKDIR /app

# Copy the Rust build files
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./src/ /app/src

# Run the example
CMD ["cargo", "test"]
