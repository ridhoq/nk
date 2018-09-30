FROM rust:1.29.0

WORKDIR /nuke
COPY . .

RUN cargo build --release

RUN ["/bin/bash"]
