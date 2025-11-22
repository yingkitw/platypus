//! platypus Protocol Buffer definitions.
//!
//! This crate contains the compiled Protocol Buffer messages used for
//! client-server communication in platypus.

#![allow(clippy::derive_partial_eq_without_eq)]

include!(concat!(env!("OUT_DIR"), "/platypus.rs"));

pub mod prelude {
    pub use crate::*;
}
