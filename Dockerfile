FROM rust

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --release

CMD [ "cargo run --release" ]
