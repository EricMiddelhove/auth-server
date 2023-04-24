FROM rust

WORKDIR /usr/src/myapp
COPY . .

RUN cargo clean
RUN cargo build --release

CMD [ "./target/release/auth-server" ]