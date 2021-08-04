#[cfg(feature = "default")]
mod inline {
    pub use std::thread::{spawn, yield_now, JoinHandle};
    pub use std::thread::Result as JoinResult;

    #[inline(always)]
    pub fn join<T>(handle: JoinHandle<T>) -> JoinResult<T> {
        handle.join()
    }

    #[macro_export]
    macro_rules! join {
        ( $handle:expr ) => {
            $crate::thread::join($handle)
        }
    }

    /// Spawn a new thread.
    ///
    /// Automatically moves all variable used.
    #[macro_export]
    macro_rules! spawn {
        ( { $( $tt:tt )* } ) => {
            $crate::thread::spawn(
                move || { $( $tt )* }
            )
        }
    }

    #[macro_export]
    macro_rules! yield_now {
        () => {
            $crate::thread::yield_now()
        }
    }
}

#[cfg(feature = "async_tokio")]
mod inline {
    pub use tokio::task::{self, spawn, yield_now, JoinHandle};
    #[must_use]
    pub type JoinResult<T> = Result<T, task::JoinError>;

    #[inline(always)]
    pub async fn join<T>(handle: JoinHandle<T>) -> JoinResult<T> {
        handle.await
    }

    #[macro_export]
    macro_rules! join {
        ( $handle:expr ) => {
            $crate::thread::join($handle).await
        }
    }

    /// Spawn a new thread.
    ///
    /// Automatically moves all variable used.
    #[macro_export]
    macro_rules! spawn {
        ( { $( $tt:tt )* } ) => {
            $crate::thread::spawn(
                async move { $( $tt )* }
            )
        }
    }

    #[macro_export]
    macro_rules! yield_now {
        () => {
            $crate::thread::yield_now().await
        }
    }
}

#[cfg(feature = "permutation_testing")]
mod inline {
    pub use loom::thread::{spawn, yield_now, JoinHandle};
    pub use std::thread::Result as JoinResult;

    #[inline(always)]
    pub fn join<T>(handle: JoinHandle<T>) -> JoinResult<T> {
        handle.join()
    }

    #[macro_export]
    macro_rules! join {
        ( $handle:expr ) => {
            $crate::thread::join($handle)
        }
    }

    /// Spawn a new thread.
    ///
    /// Automatically moves all variable used.
    #[macro_export]
    macro_rules! spawn {
        ( { $( $tt:tt )* } ) => {
            $crate::thread::spawn(
                move || { $( $tt )* }
            )
        }
    }

    #[macro_export]
    macro_rules! yield_now {
        () => {
            $crate::thread::yield_now()
        }
    }
}

pub use inline::*;
