# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# # # Copy the local Cargo.toml and src/ directory to the container
# # # COPY Cargo.toml .
# # # COPY src ./src

# # RUN apt-get update -y
# # RUN apt-get install -y iputils-ping
# # RUN ping risczero.com

# # RUN curl -L https://risczero.com/install
# # RUN curl -L https://risczero.com/install | bash
# # RUN curl -L https://risczero.com/install | /bin/sh

# RUN apt-get update && apt-get install -y bash

# # RUN curl -L https://risczero.com/install | bash

# # # RUN ./install
# # # RUN cat install | bash

# # # ENV RISC0_DIR="$HOME/.risc0"
# # # ENV RISC0_BIN_DIR="$RISC0_DIR/bin"
# # # ENV BIN_PATH="$RISC0_BIN_DIR/rzup"
# # # # ENV PATH="$PATH:$RISC0_BIN_DIR"
# # # ENV PATH="$PATH:$BIN_PATH"

# # # ENV PATH="$PATH:/opt/gtk/bin"

# # # RUN echo "$PATH" | grepp rzup
# # # SHELL ["/bin/bash", "-c"]
# # # RUN bash -c rzup --version

# # # RUN ls $HOME

# # Download and run the install script
# RUN curl -L https://risczero.com/install -o install.sh && bash install.sh

# # Set the environment variables for rzup
# RUN export BASE_DIR=${XDG_CONFIG_HOME:-$HOME} && \
#     export RISC0_DIR=${RISC0_DIR-"$BASE_DIR/.risc0"} && \
#     export RISC0_BIN_DIR="$RISC0_DIR/bin" && \
#     export PATH="$PATH:$RISC0_BIN_DIR" && \
#     rzup --version

# RUN rzup --version
# RUN rzup
RUN apt-get update
RUN cargo install cargo-binstall
RUN cargo binstall cargo-risczero -y
RUN cargo risczero install

COPY . .

# Build the Rust application
# RUN cargo build --release

# # Expose the port that the server will run on
# EXPOSE 8080

# Set the default command to run the built server
# CMD ["target/release/my_rust_server"]
CMD cargo run
