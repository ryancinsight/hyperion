#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod coefficient;
mod error;
pub mod quantity;
pub mod reference;
pub mod transport;
mod validation;

pub use error::{TransportError, TransportLaw, ValueConstraint, ValueKind};
