use core::fmt::Display;
use std::error::Error as StdError;

use anyhow::Error as AnyhowError;
use axum::{
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use itertools::Itertools as _;
use thiserror::Error;

use crate::misc::Direction;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read {direction} body for {uri}")]
    InvalidBody {
        direction: Direction,
        uri: Uri,
        source: AnyhowError,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        self.status_code().into_response()
    }
}

impl Error {
    // `anyhow::Error` prints the chain of sources if the alternate flag is specified.
    // Impls generated by `thiserror::Error` ignore the alternate flag. See:
    // - <https://github.com/dtolnay/thiserror/issues/78>
    // - <https://github.com/dtolnay/thiserror/issues/98>
    // - <https://github.com/dtolnay/thiserror/issues/214>
    pub fn format_sources(&self) -> impl Display + '_ {
        self.sources().format(": ")
    }

    // `StdError::sources` is not stable as of Rust 1.76.0.
    fn sources(&self) -> impl Iterator<Item = &dyn StdError> {
        let mut error: Option<&dyn StdError> = Some(self);

        core::iter::from_fn(move || {
            let source = error?.source();
            core::mem::replace(&mut error, source)
        })
    }

    const fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidBody { .. } => StatusCode::BAD_REQUEST,
        }
    }
}