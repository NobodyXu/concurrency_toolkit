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

    pub fn read<T>(rwlock: &RwLock<T>) -> LockResult<RwLockReadGuard<'_, T>> {
        rwlock.read()
    }

    pub fn write<T>(rwlock: &RwLock<T>) -> LockResult<RwLockWriteGuard<'_, T>> {
        rwlock.write()
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! read {
        ( $lock:expr ) => {
            $crate::sync::read($lock)
        };
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! write {
        ( $lock:expr ) => {
            $crate::sync::write($lock)
        };
    }
}

#[cfg(feature = "async_tokio")]
mod state_sync {
    pub use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
    pub type LockResult<T> = Result<T, ()>;
    pub type TryLockResult<T> = Result<T, tokio::sync::TryLockError>;

    pub async fn read<T>(rwlock: &RwLock<T>) -> LockResult<RwLockReadGuard<'_, T>> {
        Ok(rwlock.read().await)
    }

    pub async fn write<T>(rwlock: &RwLock<T>) -> LockResult<RwLockWriteGuard<'_, T>> {
        Ok(rwlock.write().await)
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! read {
        ( $lock:expr ) => {
            $crate::sync::read($lock).await
        };
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! write {
        ( $lock:expr ) => {
            $crate::sync::write($lock).await
        };
    }
}

#[cfg(feature = "permutation_testing")]
mod state_sync {
    pub use loom::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult, TryLockResult};

    pub fn read<T>(rwlock: &RwLock<T>) -> LockResult<RwLockReadGuard<'_, T>> {
        rwlock.read()
    }

    pub fn write<T>(rwlock: &RwLock<T>) -> LockResult<RwLockWriteGuard<'_, T>> {
        rwlock.write()
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! read {
        ( $lock:expr ) => {
            $crate::sync::read($lock)
        };
    }

    /// Must use [`maybe_async`](/maybe_async) keyword when using this macro
    #[macro_export]
    macro_rules! write {
        ( $lock:expr ) => {
            $crate::sync::write($lock)
        };
    }
}

pub use state_sync::*;

#[cfg(test)]
mod tests {
    use super::RwLock;
    use crate::{maybe_async, read, write};

    #[crate::test(
        feature = "is_sync",
        async(not(feature = "is_sync"), tokio::test)
    )]
    async fn test_rwlock() {
        crate::run_test!({
            let rwlock = RwLock::new(());
            {
                let _guard = read!(&rwlock).unwrap();
            }
            {
                let _guard = write!(&rwlock).unwrap();
            }
        });
    }
}
