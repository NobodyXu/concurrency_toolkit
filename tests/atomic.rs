extern crate concurrency_toolkit;

use core::ptr::null_mut;
use concurrency_toolkit::atomic::*;

#[concurrency_toolkit::test]
fn test_assert_store() {
    let null = null_mut();

    let atomic = AtomicPtr::new(null);

    let mut val = 1;
    assert_store_ptr(&atomic, null, &mut val, Ordering::AcqRel, Ordering::Release);
}
