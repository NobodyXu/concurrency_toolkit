#[cfg(not(features = "permutation_testing"))]
mod state_storage {
    pub use std::sync::Arc;
}
#[cfg(features = "permutation_testing")]
mod state_storage {
    pub use loom::sync::Arc;
}
pub use state_storage::*;

#[cfg(not( any(feature = "async_tokio", features = "permutation_testing") ))]
mod state_sync {
    pub use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult};

    pub fn read<T: ?Sized>(rwlock: &RwLock<T>) -> LockResult<RwLockReadGuard<'_, T>> {
        rwlock.read()
    }

    pub fn write<T: ?Sized>(rwlock: &RwLock<T>) -> LockResult<RwLockWriteGuard<'_, T>> {
        rwlock.write()
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
mod state_sync {
    pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
    pub type LockResult<T> = Result<T, ()>;

    pub async fn read<T: ?Sized>(rwlock: &RwLock<T>) -> LockResult<RwLockReadGuard<'_, T>> {
        Ok(rwlock.read())
    }

    pub async fn write<T: ?Sized>(rwlock: &RwLock<T>) -> LockResult<RwLockWriteGuard<'_, T>> {
        Ok(rwlock.write())
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
mod state_sync {
    pub use loom::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult};

    pub fn read<T: ?Sized>(rwlock: &RwLock<T>) -> LockResult<RwLockReadGuard<'_, T>> {
        rwlock.read()
    }

    pub fn write<T: ?Sized>(rwlock: &RwLock<T>) -> LockResult<RwLockWriteGuard<'_, T>> {
        rwlock.write()
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

pub use state_sync::*;
