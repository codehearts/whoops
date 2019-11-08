FROM rust:1.39.0-slim-buster AS build

WORKDIR /app

# Install build dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libssl-dev pkg-config

# Add the wasm32 target and install wasm-pack
RUN rustup target add wasm32-unknown-unknown\
    && cargo install wasm-pack

# Copy the Rust build files
COPY ./Cargo.toml /app/Cargo.toml
COPY ./Cargo.lock /app/Cargo.lock
COPY ./src/ /app/src

# Build the project
RUN wasm-pack build --dev --out-name series --target nodejs

FROM build AS release

# Install build dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        nodejs \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy the Node.js example
COPY ./index.js /app/index.js
COPY --from=build /app/pkg /app/pkg

# Run the example
CMD ["node", "index.js"]
