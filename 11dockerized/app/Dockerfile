FROM rust:1.65

RUN cargo install cargo-watch

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD ["cargo", "run"]

#FROM rust:1.65
#
## RUN cargo install diesel_cli --no-default-features --features postgres
#
#RUN cargo install cargo-watch
#
#
#WORKDIR /usr/src/app
#
#COPY . .
#
#RUN cargo build
#
#EXPOSE 8000
#
#VOLUME ["/usr/local/cargo"]