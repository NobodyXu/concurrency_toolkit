#[cfg(all(feature = "async_tokio", feature = "permutation_testing"))]
compile_error!("feature async cannot be used with feature permutation_testing");

#[cfg(feature = "async_tokio")]
pub extern crate tokio;

#[cfg(feature = "permutation_testing")]
pub extern crate loom;

pub mod sync;
