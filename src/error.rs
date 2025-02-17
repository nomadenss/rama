//! Error types for rama.

use std::{
    error::Error as StdError,
    fmt,
    ops::{Deref, DerefMut},
};

/// Alias for a type-erased error type.
pub type BoxError = Box<dyn StdError + Send + Sync>;

/// Errors that can happen when using rama.
#[derive(Debug)]
pub struct Error {
    inner: BoxError,
}

impl Error {
    /// Create a new `Error` from a boxable error.
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self {
            inner: error.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl AsRef<dyn StdError + Send + Sync> for Error {
    fn as_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
        &**self
    }
}

impl AsRef<dyn StdError> for Error {
    fn as_ref(&self) -> &(dyn StdError + 'static) {
        &**self
    }
}

impl Deref for Error {
    type Target = dyn StdError + Send + Sync + 'static;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.inner
    }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self {
            inner: Box::new(error),
        }
    }
}

impl From<Error> for BoxError {
    fn from(error: Error) -> Self {
        error.inner
    }
}
