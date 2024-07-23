# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# # Copy the local Cargo.toml and src/ directory to the container
# # COPY Cargo.toml .
# # COPY src ./src

# RUN apt-get update -y
# RUN apt-get install -y iputils-ping
# RUN ping risczero.com

# RUN curl -L https://risczero.com/install
# RUN curl -L https://risczero.com/install | bash
# RUN curl -L https://risczero.com/install | /bin/sh

RUN apt-get update && apt-get install -y bash

RUN curl -L https://risczero.com/install | bash

# RUN ./install
# RUN cat install | bash

# ENV RISC0_DIR="$HOME/.risc0"
# ENV RISC0_BIN_DIR="$RISC0_DIR/bin"
# ENV BIN_PATH="$RISC0_BIN_DIR/rzup"
# # ENV PATH="$PATH:$RISC0_BIN_DIR"
# ENV PATH="$PATH:$BIN_PATH"

# ENV PATH="$PATH:/opt/gtk/bin"

# RUN echo "$PATH" | grepp rzup
# SHELL ["/bin/bash", "-c"]
# RUN bash -c rzup --version

# RUN ls $HOME

RUN rzup --version
RUN rzup

COPY . .

# Build the Rust application
# RUN cargo build --release

# # Expose the port that the server will run on
# EXPOSE 8080

# Set the default command to run the built server
# CMD ["target/release/my_rust_server"]
CMD cargo run
