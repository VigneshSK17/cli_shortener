FROM rust:latest

WORKDIR /myportfolio

COPY . .

RUN cargo build

CMD ["./target/debug/cli_shortener", "-v", "--host=0.0.0.0", "start"]

EXPOSE 8080
