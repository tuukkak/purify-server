FROM rust:1.27.1

WORKDIR /usr/src/app
COPY . .

RUN rustc app.rs

EXPOSE 3000

CMD ["./app"]
