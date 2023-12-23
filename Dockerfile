#Build Stage
FROM rust:latest as builder

#TEMP DIR
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./static ./static
COPY ./src ./src
#BUILD
RUN cargo build --release --verbose

# Final Stage
FROM debian:stable-slim

RUN apt-get update
RUN apt-get upgrade
RUN apt-get install libssl3

#ARGS
ARG MONGO_ADRESS="127.0.0.1"
ARG REDIS_ADRESS="127.0.0.1"
ARG LOGIN_REDIS="example"
ARG PASSWORD_REDIS="example"
ARG LOGIN_MONGO="example"
ARG PASSWORD_MONGO="example"
ARG SMTP_LOGIN="Example"
ARG SMTP_PASSWORD="Example"
ARG SMTP_ADRESS="Example"



#MAIN_WEB_PORT
EXPOSE 8080
#MONGODB
EXPOSE 27017
#REDIS
EXPOSE 6379

#ENV
ENV RUST_LOG=info 
ENV MONGO_ADRESS=$MONGO_ADRESS 
ENV REDIS_ADRESS=$REDIS_ADRESS 
ENV LOGIN_REDIS=$LOGIN_REDIS 
ENV PASSWORD_REDIS=$PASSWORD_REDIS 
ENV LOGIN_MONGO=$LOGIN_MONGO 
ENV PASSWORD_MONGO=$PASSWORD_MONGO
ENV SMTP_LOGIN=$SMTP_LOGIN
ENV SMTP_PASSWORD=$SMTP_PASSWORD
ENV SMTP_ADRESS=$SMTP_ADRESS

#MAIN DIR .
WORKDIR /app
COPY --from=builder /app/target/release/back /app/back

#RUN
CMD ["./back", "--mongo-adress", "${MONGO_ADRESS}", "--redis-adress", "${REDIS_ADRESS}", "--redis-login", "${LOGIN_REDIS}", "--redis-password", "${PASSWORD_REDIS}", "--mongo-login", "${LOGIN_MONGO}", "--mongo-password", "${PASSWORD_MONGO}", "--smtp-login", "${SMTP_LOGIN}", "--smtp-password", "${SMTP_PASSWORD}", "--smtp-adress", "${SMTP_ADRESS}"]


