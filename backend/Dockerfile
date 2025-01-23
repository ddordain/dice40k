FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/dice40k"]
EXPOSE 8000
