#!/bin/sh -ex

cd $(dirname `realpath $0`)

export RUST_BACKTRACE=1

for feature in default async_tokio; do
    cargo test --no-default-features --features $feature
    cargo test --release --no-default-features --features $feature
done

cargo test --release --no-default-features --features permutation_testing
