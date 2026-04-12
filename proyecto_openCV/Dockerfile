FROM rust:1.87-bookworm

RUN apt-get update && apt-get install -y \
    libopencv-dev \
    clang \
    libclang-dev \
    cmake \
    pkg-config

WORKDIR /app
