#[cfg(not(features = "permutation_testing"))]
mod state_storage {
    pub use std::sync::Arc;
}
#[cfg(features = "permutation_testing")]
mod state_storage {
    pub use loom::sync::Arc;
}
pub use state_storage::*;

#[cfg(feature = "default")]
mod state_sync {
    pub use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult, TryLockResult};

    #[inline(always)]
    pub fn obtain_read_lock<T>(rwlock: &RwLock<T>)
        -> LockResult<RwLockReadGuard<'_, T>>
    {
        rwlock.read()
    }

    #[inline(always)]
    pub fn obtain_write_lock<T>(rwlock: &RwLock<T>)
        -> LockResult<RwLockWriteGuard<'_, T>>
    {
        rwlock.write()
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! obtain_read_lock {
        ( $lock:expr ) => {
            $crate::sync::obtain_read_lock($lock)
        };
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! obtain_write_lock {
        ( $lock:expr ) => {
            $crate::sync::obtain_write_lock($lock)
        };
    }
}

#[cfg(feature = "async_tokio")]
mod state_sync {
    pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
    pub type LockResult<T> = Result<T, ()>;
    pub type TryLockResult<T> = Result<T, tokio::sync::TryLockError>;

    #[inline(always)]
    pub async fn obtain_read_lock<T>(rwlock: &RwLock<T>)
        -> LockResult<RwLockReadGuard<'_, T>>
    {
        Ok(rwlock.read().await)
    }

    #[inline(always)]
    pub async fn obtain_write_lock<T>(rwlock: &RwLock<T>)
        -> LockResult<RwLockWriteGuard<'_, T>>
    {
        Ok(rwlock.write().await)
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! obtain_read_lock {
        ( $lock:expr ) => {
            $crate::sync::obtain_read_lock($lock).await
        };
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! obtain_write_lock {
        ( $lock:expr ) => {
            $crate::sync::obtain_write_lock($lock).await
        };
    }
}

#[cfg(feature = "permutation_testing")]
mod state_sync {
    pub use loom::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult, TryLockResult};

    #[inline(always)]
    pub fn obtain_read_lock<T>(rwlock: &RwLock<T>)
        -> LockResult<RwLockReadGuard<'_, T>>
    {
        rwlock.read()
    }

    #[inline(always)]
    pub fn obtain_write_lock<T>(rwlock: &RwLock<T>)
        -> LockResult<RwLockWriteGuard<'_, T>>
    {
        rwlock.write()
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! obtain_read_lock {
        ( $lock:expr ) => {
            $crate::sync::obtain_read_lock($lock)
        };
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! obtain_write_lock {
        ( $lock:expr ) => {
            $crate::sync::obtain_write_lock($lock)
        };
    }
}

pub use state_sync::*;
