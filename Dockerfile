FROM ubuntu:latest
COPY ./target/release/actix-serve .
EXPOSE 3000
CMD ["./actix-serve"]
