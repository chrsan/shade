FROM debian:buster-slim

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
  clang \
  libc++-dev \
  libc++abi-dev \
  ninja-build \
  git \
  && rm -rf /var/lib/apt/lists/*
