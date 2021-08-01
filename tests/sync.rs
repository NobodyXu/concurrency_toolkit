extern crate concurrency_toolkit;

use concurrency_toolkit::sync::RwLock;
use concurrency_toolkit::{obtain_read_lock, obtain_write_lock};

#[concurrency_toolkit::test]
fn test_rwlock() {
    let rwlock = RwLock::new(());
    {
        let _guard = obtain_read_lock!(&rwlock).unwrap();
    }
    {
        let _guard = obtain_write_lock!(&rwlock).unwrap();
    }
}
