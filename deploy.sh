#!/bin/bash

# Build the project
cargo build

# Add the target/debug directory to the PATH
export PATH=$PWD/target/debug:$PATH

# Initialize the Linera network helper and spawn/read wallet variables, then start the Linera network
eval "$(linera net helper)"
linera_spawn_and_read_wallet_variables \
    linera net up \
        --extra-wallets 1 \
        --testing-prng-seed 37

# Define chain and owner variables
CHAIN_0=e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65
OWNER_0=7136460f0c87ae46f966f898d494c4b40c4ae8c527f4d1c0b1fa0f7cff91d20f
CHAIN_1=1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03
OWNER_1=b4f8586041a07323bd4f4ed2d758bf1b9a977eabfd4c00e2f12d08a0899485fd

# Publish and create the application, capturing the APP_ID
APP_ID=$(linera --with-wallet 0 project publish-and-create \
           --json-argument "{\"players\": [\"$OWNER_0\", \"$OWNER_1\"]}")

# Start the Linera services and keep them running
linera --with-wallet 0 service --port 8080 &
linera --with-wallet 1 service --port 8081 &

# Wait for background processes to finish
wait