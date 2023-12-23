#Build Stage
FROM rust as builder

#TEMP DIR
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./static ./static
COPY ./src ./src
#BUILD
RUN cargo build --release --verbose

# Final Stage
FROM alpine:latest

#Update and add some dependencies
RUN apk update && apk upgrade && \
    apk add \
    openssl-dev\
    ca-certificates
#ARGS
ARG mongo_address=mongo
ARG redis_address=redis

ARG login_redis
ARG password_redis

ARG login_mongo=root
ARG password_mongo=example

ARG smtp_login
ARG smtp_password
ARG smtp_address
#trace, info, error, debug
ARG RUST_LOG=info 


#MAIN_WEB_PORT
EXPOSE 8080

#ENV
ENV MONGO_ADDRESS $mongo_address
ENV MONGO_LOGIN $login_mongo
ENV MONGO_PASSWORD $password_mongo
ENV RUST_LOG $RUST_LOG

ENV REDIS_ADDRESS $redis_address
ENV REDIS_LOGIN $login_redis
ENV REDIS_PASSWORD $password_redis

ENV SMTP_LOGIN $smtp_login
ENV SMTP_PASSWORD $smtp_password
ENV SMTP_ADDRESS $smtp_address

#MAIN DIR .
WORKDIR /app
COPY --from=builder /app/static /app/static
COPY --from=builder /app/target/release/back /app/back

#RUN
CMD ["./back"]



