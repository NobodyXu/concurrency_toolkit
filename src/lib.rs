#[cfg(not( any(feature = "default", feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At least one feature flag needs to be enabled");

#[cfg(all( feature = "default", any(feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At most one feature flag can to be enabled");

#[cfg(all(feature = "async_tokio", feature = "permutation_testing"))]
compile_error!("feature async cannot be used with feature permutation_testing");

extern crate proc_macro_test;

/// `maybe_async::maybe_async` automatically removes `async`-related keywords
/// if feature "async_tokio" is not enabled.
pub extern crate maybe_async;

/// Automatically start async runtime or call `loom::model` if required:
///
/// ```no_run
/// #[concurrency_toolkit::test]
/// fn test() {
///     // ...
/// }
/// ```
///
/// However, unlike `maybe_async::maybe_async`, this proc macro requires the function
/// to not be declared as `async` due to implementation detail
/// (`syn` doesn't provides an easy way to parse `async function), but it still can
/// remove `async`-related keywords just like `maybe_async::maybe_async`.
pub use proc_macro_test::test;

#[cfg(feature = "async_tokio")]
pub extern crate tokio;

#[cfg(feature = "permutation_testing")]
pub extern crate loom;

pub mod sync;
pub mod atomic;
