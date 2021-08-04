extern crate concurrency_toolkit;

use concurrency_toolkit::sync::{Arc, RwLock};
use concurrency_toolkit::{obtain_read_lock, obtain_write_lock, spawn, join, yield_now};

#[concurrency_toolkit::test]
fn test_thread() {
    let rwlock = Arc::new(RwLock::new(()));

    let rwlock_cloned = Arc::clone(&rwlock);
    let handle1 = spawn!({
        yield_now!();
        let _guard = obtain_read_lock!(&rwlock_cloned).unwrap();
    });

    let handle2 = spawn!({
        yield_now!();
        let _guard = obtain_write_lock!(&rwlock).unwrap();
    });

    join!(handle1).unwrap();
    join!(handle2).unwrap();
}
