#!/usr/bin/env bash 

# install prebuilt binaries
cargo install cargo-binstall
cargo binstall trunk -y
cargo binstall worker-build -y

# build static assets (index.html and wasm frontend)
cd mnemnos-wasm
trunk build --release true
cd ..

# build wasm backend (api endpoints)
cd mnemnos-worker
worker-build --release
cd ..
