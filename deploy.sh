#!/bin/bash

set -e

# Deploy programs
echo -e "\e[32mDEPLOYING gary_pool_program PROGRAM\e[0m"
cd gary-pool && \
cargo build-sbf && \
solana program deploy target/deploy/gary_pool_program.so --program-id ../keypairs/poo1BN3ttEArDXLjfKHXvgMYHv7BwyccKG3Jzyb9hSp.json

echo -e "\e[32mDEPLOYING gary_boost PROGRAM\e[0m"
cd ../gary-boost && \
cargo build-sbf && \
solana program deploy target/deploy/gary_boost.so --program-id ../keypairs/boost4Zr7jTvLHtc4H1B9m5LxFcM5qNMyGPBNuv55eo.json && \
cd cli && cargo run -- initialize

echo -e "\e[32mDEPLOYING gary PROGRAM\e[0m"
cd ../.. && \
cargo build-sbf && \
solana program deploy target/deploy/gary.so --program-id keypairs/garykparLECYt95RvvpBwnHTmGWbXEAyzXtTcj3VB4J.json && \
cd gary-cli && \
cargo run --features admin -- initialize