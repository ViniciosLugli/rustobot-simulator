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

# Check if the COMPOSE_PROFILES is production
CMD if [ "$COMPOSE_PROFILES" = "production" ]; then \
	# Run the interface in release mode
	trunk--config Trunk.toml --release serve; \
	elif [ "$COMPOSE_PROFILES" = "test" ]; then \
	# Run tests in interface for CI
	cargo test; \
	else \
	# Run the interface in debug mode (watch mode)
	trunk --config Trunk.toml serve; \
	fi
