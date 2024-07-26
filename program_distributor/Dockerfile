# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# # Copy the local Cargo.toml and src/ directory to the container
# # COPY Cargo.toml .
# # COPY src ./src

COPY . .

# Build the Rust application
# RUN cargo build --release

# # Expose the port that the server will run on
# EXPOSE 8080

# Set the default command to run the built server
# CMD ["target/release/my_rust_server"]
CMD cargo run
