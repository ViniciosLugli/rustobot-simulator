# Use the slim bullseye Rust image as the base ( because it's smaller than the latest image )
FROM rust:slim-bullseye

# Install the pkg-config and libssl-dev packages
RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev

# Install the wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Install the Trunk CLI tool
RUN cargo install trunk wasm-bindgen-cli

# Set the working directory to /interface
WORKDIR /interface

# Copy the all files from the host to the container (except the ones in .dockerignore)
COPY . .