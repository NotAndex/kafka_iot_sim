FROM rust:1.69 as builder
RUN apt-get update && apt-get install -y cmake build-essential && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/iot_data_gen
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN echo cmake --version
COPY --from=builder /usr/local/cargo/bin/iot_data_gen /usr/local/bin/iot_data_gen

ENTRYPOINT ["iot_data_gen"]
