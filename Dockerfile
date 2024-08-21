FROM ubuntu:latest
COPY ./target/release/actix-serve .
EXPOSE 8080
CMD ["./actix-serve"]
