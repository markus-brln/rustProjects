FROM rust:1.70

WORKDIR /usr/src/app

COPY . .

RUN #cargo install --path .
RUN cargo build --release

EXPOSE 8000

CMD ["./target/release/deployment"]
#CMD ["app"]
