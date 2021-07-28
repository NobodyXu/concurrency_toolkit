#[cfg(not( any(feature = "default", feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At least one feature flag needs to be enabled");

#[cfg(all( feature = "default", any(feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At most one feature flag can to be enabled");

#[cfg(all(feature = "async_tokio", feature = "permutation_testing"))]
compile_error!("feature async cannot be used with feature permutation_testing");

extern crate maybe_async;
/// This ttribute macro automatically removes `async`-related keywords if feature "async_tokio"
/// is not enabled.
pub use maybe_async::maybe_async;

#[cfg(feature = "async_tokio")]
pub extern crate tokio;

#[cfg(feature = "permutation_testing")]
pub extern crate loom;

pub mod sync;
pub mod atomic;
