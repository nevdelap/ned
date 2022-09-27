FROM rust:latest
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup component add rustfmt
RUN rustup component add clippy
ENV HOME=/home/ned
ENV CARGO_HOME="$HOME/.cargo"
WORKDIR /home/ned
