# Use the official Rust image as the base image
# FROM rust:latest
FROM ubuntu:latest

# Set the working directory inside the container
WORKDIR /app

# # # Copy the local Cargo.toml and src/ directory to the container
# # # COPY Cargo.toml .
# # # COPY src ./src

RUN apt-get update

RUN apt install curl -y

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc


RUN apt install build-essential autoconf automake libtool pkg-config -y
# RUN apt install libprotobuf-dev protobuf-compiler -y
RUN apt install protobuf-compiler -y
RUN apt install libprotobuf-dev -y
# RUN apt install protoc -y
RUN apt install xz-utils -y
RUN apt install gawk -y
RUN apt install python3 -y
RUN apt-get install bash -y

RUN apt-get install -y libssl-dev

# ENV PATH="$PATH:/.cargo/bin"

# RUN /.cargo/bin install cargo-binstall

# RUN cargo install cargo-binstall
RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo install cargo-binstall"]

# RUN cargo binstall cargo-risczero -y
RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo binstall cargo-risczero -y"]

# RUN cargo risczero install
RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo risczero install"]

# PROTOC_ZIP=protoc-3.15.8-linux-x86_64.zip
# curl -OL https://github.com/google/protobuf/releases/download/v3.15.8/$PROTOC_ZIP
# sudo unzip -o $PROTOC_ZIP -d /usr/local bin/protoc
# sudo unzip -o $PROTOC_ZIP -d /usr/local include/*
# rm -f $PROTOC_ZIP

# # COPY . .
# # COPY Cargo.lock Cargo.toml src ./
# COPY Cargo.lock Cargo.toml ./src /app/
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY ./src ./src

# Build the Rust application
# RUN cargo build --release

# # Expose the port that the server will run on
# EXPOSE 8080

# Set the default command to run the built server
# CMD ["target/release/my_rust_server"]
# CMD cargo run
RUN ["/bin/bash", "-c", "~/.cargo/bin/cargo run"]
