FROM rust:1.29.0

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release

RUN ["/bin/bash"]
