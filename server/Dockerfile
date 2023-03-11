# Use the slim bullseye Rust image as the base ( because it's smaller than the latest image )
FROM rust:slim-bullseye

# Install the pkg-config and libssl-dev packages
RUN apt-get --yes update && apt-get --yes install pkg-config libssl-dev

# Install cargo-watch for hot reloading if the ENVIRONMENT variable is not set to "production"
RUN if [ "$ENVIRONMENT" != "production" ]; then \
	cargo install cargo-watch; \
	fi

# Set the working directory to /server
WORKDIR /server

# Copy the all files from the host to the container (except the ones in .dockerignore)
COPY . .

# Check if the COMPOSE_PROFILES is production
CMD if [ "$COMPOSE_PROFILES" = "production" ]; then \
	# Run the server in release mode
	cargo run --release; \
	elif [ "$COMPOSE_PROFILES" = "test" ]; then \
	# Run tests in server for CI
	cargo test; \
	else \
	# Run the server in debug mode (watch mode)
	cargo watch -w src -x 'run'; \
	fi