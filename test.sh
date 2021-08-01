#!/bin/sh -ex

cd $(dirname `realpath $0`)

for feature in default async_tokio permutation_testing; do
    cargo test --no-default-features --features $feature
    cargo test --release --no-default-features --features $feature
done
