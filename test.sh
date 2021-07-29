#!/bin/sh -ex

for feature in default async_tokio permutation_testing; do
    cargo test --no-default-features --features $feature
    cargo test --release --no-default-features --features $feature
done
