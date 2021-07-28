#[cfg(not( any(feature = "default", feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At least one feature flag needs to be enabled");

#[cfg(all( feature = "default", any(feature = "async_tokio", feature = "permutation_testing") ))]
compile_error!("At most one feature flag can to be enabled");

#[cfg(all(feature = "async_tokio", feature = "permutation_testing"))]
compile_error!("feature async cannot be used with feature permutation_testing");

extern crate maybe_async;
/// `maybe_async` automatically removes `async`-related keywords if feature "async_tokio"
/// is not enabled.
pub use maybe_async::maybe_async;

/// `test` needs be used with `run_test` to write tests:
///
/// ```no_run
/// #[concurrency_toolkit::test(
///     feature = "is_sync",
///     async(not(feature = "is_sync"), tokio::test)
/// )]
/// async fn test() {
///     crate::run_test!({
///         ...
///     });
/// }
/// ```
pub use maybe_async::test;

#[cfg(feature = "async_tokio")]
pub extern crate tokio;

#[cfg(feature = "permutation_testing")]
pub extern crate loom;

pub mod sync;
pub mod atomic;

#[cfg(not(feature = "permutation_testing"))]
mod inline {
    /// run_test needs to be called inside test, like this
    ///
    /// ```no_run
    /// #[concurrency_toolkit::test(
    ///     feature = "is_sync",
    ///     async(not(feature = "is_sync"), tokio::test)
    /// )]
    /// async fn test() {
    ///     crate::run_test!({
    ///         ...
    ///     });
    /// }
    /// ```
    #[macro_export]
    macro_rules! run_test {
        ( { $( $tt:tt )* } ) => {
            $( $tt )*
        };
    }
}

#[cfg(feature = "permutation_testing")]
mod inline {
    /// run_test needs to be called inside test, like this
    ///
    /// ```no_run
    /// #[concurrency_toolkit::test(
    ///     feature = "is_sync",
    ///     async(not(feature = "is_sync"), tokio::test)
    /// )]
    /// async fn test() {
    ///     crate::run_test!({
    ///         ...
    ///     });
    /// }
    /// ```
    #[macro_export]
    macro_rules! run_test {
        ( { $( $tt:tt )* } ) => {
            loom::model(
                || { $( $tt )* }
            )
        };
    }
}
