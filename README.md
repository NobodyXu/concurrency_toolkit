# concurrency_toolkit

[![Rust](https://github.com/NobodyXu/concurrency_toolkit/actions/workflows/rust.yml/badge.svg)](https://github.com/NobodyXu/concurrency_toolkit/actions/workflows/rust.yml)

[![crate.io downloads](https://img.shields.io/crates/d/concurrency_toolkit)](https://crates.io/crates/concurrency_toolkit)

Easy switching between std::sync, tokio::sync and loom::sync

## Usage

This crate provides two modules `sync` and `atomic`.

Module `atomic` provides atomic primitives that are typedefs to `core::sync::atomic` by default and `loom::sync::atomic` when feature `permutation_testing` is enabled.

Module `sync` is similar to module `atomic` -- it provides primitives from `std::sync` on `default`, `tokio::sync` on `async_tokio` and `loom::sync` on `permutation_testing`.

Currently, it only provides `Arc` and `RwLock`, but more primitives will be added in future.

It also comes with unified typedef to deal with the difference between these libraries:
 - `RwLockReadGuard`
 - `RwLockWriteGuard`
 - `LockResult`
 - `TryLockResult`

There is also macros to deal with the difference between calling to asynchronous and synchronous function in Rust:
 - `obtain_read_lock!(reference to rwlock)`
 - `obtain_write_lock!(reference to rwlock)`

To create a function uses `sync::RwLock`, uses `concurrency_toolkit::maybe_async::maybe_async`, which automatically removes all `async`-related keywords if `async_tokio` is not used.

Example for using `concurrency_toolkit::maybe_async::maybe_async`:

```rust
use concurrency_toolkit::maybe_async::maybe_async;
use concurrency_toolkit::sync::{RwLock, obtain_read_lock, obtain_write_lock};

#[maybe_async]
fn get(rwlock: &RwLock<i32>) -> i32 {
    *obtain_read_lock!(rwlock).unwrap()
}

#[maybe_async]
fn set(rwlock: &RwLock<i32>, val: i32) {
    *obtain_write_lock!(rwlock).unwrap() = val;
}

#[maybe_async]
fn add(rwlock: &RwLock<i32>, val: i32) {
    set(rwlock, get(rwlock).await + val).await;
}
```

## Testing

This crate provides proc macro `concurrency_toolkit::test` to run your test:

```
#[concurrency_toolkit::test]
fn test() {
    // ...
}
```

It will automatically start async runtime or call `loom::model` for you if required.

However, unlike `maybe_async::maybe_async`, this proc macro requires the function to not be declared as `async` due to implementation detail (`syn` doesn't provides an easy way to parse `async function), but it still can remove `async`-related keywords just like `maybe_async::maybe_async`.

## Feature

This crate currently supports 3 features:
 - `default` where `std::sync` is used;
 - `async_tokio` where `tokio::sync` is used in mod `sync` and `std::sync::atomic` is used for mod `atomic`;
 - `permutation_testing` where `loom::sync` is used.

These features are exclusive, so I recommend you to use `default-features = false` when adding this crate as a dependency and let the user to explicitly opt-in features they want.
