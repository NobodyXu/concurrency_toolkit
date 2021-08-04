#[cfg(feature = "default")]
mod inline {
    pub use std::thread::{spawn, JoinHandle};
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

    #[macro_export]
    macro_rules! spawn {
        ( { $( $tt:tt )* } ) => {
            $crate::thread::spawn(
                move || { $( $tt )* }
            )
        }
    }
}

#[cfg(feature = "async_tokio")]
mod inline {
    pub use tokio::task::{self, spawn, JoinHandle};
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

    #[macro_export]
    macro_rules! spawn {
        ( { $( $tt:tt )* } ) => {
            $crate::thread::spawn(
                async move { $( $tt )* }
            )
        }
    }
}

#[cfg(feature = "permutation_testing")]
mod inline {
    pub use loom::thread::{spawn, JoinHandle};
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

    #[macro_export]
    macro_rules! spawn {
        ( { $( $tt:tt )* } ) => {
            $crate::thread::spawn(
                move || { $( $tt )* }
            )
        }
    }
}

pub use inline::*;
