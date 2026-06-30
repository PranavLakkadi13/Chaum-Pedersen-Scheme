FROM rust:latest

WORKDIR /zkp_server/

COPY . .

RUN apt update 

RUN apt-get install -y protobuf-compiler

RUN cargo build --release --bin server --bin client

CMD ["./target/release/server"]
