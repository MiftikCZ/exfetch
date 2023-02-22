#!/bin/bash
cargo build
cp ./target/debug/exfetch ./
echo "==> NOW YOU CAN USE ./exfetch COMMAND"