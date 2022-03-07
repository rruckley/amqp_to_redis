FROM rust:1.59

WORKDIR /usr/src/amqp-to-redis
COPY . .

RUN cargo install --path .

CMD ["amqp-to-redis"]
