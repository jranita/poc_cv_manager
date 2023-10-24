FROM rust:latest

WORKDIR /app/

COPY . .

RUN rustup default
RUN rustup update
RUN rustup component add clippy

# RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres
RUN cargo install cargo-watch
RUN 

CMD ["cargo", "watch", "--why", "--", "echo"]