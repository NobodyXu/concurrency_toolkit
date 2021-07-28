#[cfg(not( any(feature = "async_tokio", features = "permutation_testing") ))]
mod inline {
    pub use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    pub fn read<T: ?Sized>(rwlock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
        rwlock.read().expect("Failed to lock std::sync::RwLock for reading")
    }

    pub fn write<T: ?Sized>(rwlock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
        rwlock.write().expect("Failed to lock std::sync::RwLock for writing")
    }

    #[macro_export]
    macro_rules! read {
        ( $lock:expr ) => {
            $crate::rw_lock::read($lock)
        };
    }

    #[macro_export]
    macro_rules! write {
        ( $lock:expr ) => {
            $crate::rw_lock::write($lock)
        };
    }
}

#[cfg(feature = "async_tokio")]
mod inline {
    pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    pub async fn read<T: ?Sized>(rwlock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
        rwlock.read()
    }

    pub async fn write<T: ?Sized>(rwlock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
        rwlock.write()
    }

    #[macro_export]
    macro_rules! read {
        ( $lock:expr ) => {
            $crate::rw_lock::read($lock).await
        };
    }

    #[macro_export]
    macro_rules! write {
        ( $lock:expr ) => {
            $crate::rw_lock::write($lock).await
        };
    }
}

#[cfg(feature = "permutation_testing")]
#[macro_use]
mod inline {
    pub use loom::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

    pub fn read<T: ?Sized>(rwlock: &RwLock<T>) -> RwLockReadGuard<'_, T> {
        rwlock.read().expect("Failed to lock std::sync::RwLock for reading")
    }

    pub fn write<T: ?Sized>(rwlock: &RwLock<T>) -> RwLockWriteGuard<'_, T> {
        rwlock.write().expect("Failed to lock std::sync::RwLock for writing")
    }

    macro_rules! read {
        ( $lock:expr ) => {
            $crate::rw_lock::read($lock)
        };
    }

    macro_rules! write {
        ( $lock:expr ) => {
            $crate::rw_lock::write($lock)
        };
    }
}

pub use inline::*;
