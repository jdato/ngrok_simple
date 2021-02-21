FROM rust:1.50-buster

# Setup environment
RUN apt update
RUN apt install curl -y
RUN apt install zip -y
RUN curl https://bin.equinox.io/c/4VmDzA7iaHb/ngrok-stable-linux-arm.zip -o ngrok.zip
RUN unzip ./ngrok.zip

ENV RUST_BACKTRACE="1"

# Build dependencies
ADD Cargo.toml /app/
ADD Cargo.lock /app/
RUN cd /app && echo "fn main() { }" > build.rs && mkdir src && echo "fn main() { }" > src/main.rs && cargo build --release

# Build app - && touch *.rs && touch src/*.rs 
ADD src /app/src
RUN cd /app && cargo build --release
ENTRYPOINT [ "/app/target/release/ngrok_simple" ]