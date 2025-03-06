# The official rust release lacks a lot of necessary dependencies for risc0
FROM ubuntu:latest

# Set the working directory inside the container
WORKDIR /app

RUN apt-get update
RUN apt install curl -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc

RUN apt-get install bash build-essential pkg-config libssl-dev -y

# RUN cargo install cargo-binstall
# RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo install cargo-binstall"]
# RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo install cargo-binstall --version 1.10.7"]
RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo install cargo-binstall --version 1.10.7 --locked"]

# RUN cargo binstall cargo-risczero -y
# RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo binstall cargo-risczero -y"]
RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo binstall cargo-risczero --version 1.0.1 -y"]

# RUN cargo risczero install
RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo risczero install"]
# RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo risczero install --version 1.0.1"]

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY ./src ./src

# RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo run"]
CMD ["/bin/bash", "-c", "~/.cargo/bin/cargo run"]
