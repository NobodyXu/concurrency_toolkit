#[cfg(not(feature = "permutation_testing"))]
pub use core::sync::atomic::*;

#[cfg(feature = "permutation_testing")]
pub use loom::sync::atomic::*;
