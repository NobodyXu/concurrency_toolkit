#[cfg(not(feature = "permutation_testing"))]
pub use core::sync::atomic::*;

#[cfg(feature = "permutation_testing")]
pub use loom::sync::atomic::*;

/// atomic store with assertion on old_val equals to the one of atomic
/// before storing new_val under debug mode.
pub fn assert_store_ptr<T>(
    atomic: &AtomicPtr<T>,
    _old_val: *mut T,
    new_val: *mut T,
    _debug_order: Ordering,
    release_order: Ordering
) {
    #[cfg(debug)]
    assert!(ptr::eq(_old_val, atomic.swap(new_val, _debug_order)));
    #[cfg(not(debug))]
    atomic.store(new_val, release_order);
}
