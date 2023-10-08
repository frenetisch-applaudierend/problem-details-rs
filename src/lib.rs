//! RFC 9457 / RFC 7807 problem details for Rust applications.
//!
//! This crate implements a simple struct that can be used to
//! represent a problem details object as defined in RFC 9457
//! (which obsoletes RFC 7807).
//!
//! # Features
//!
//! - `serde`: Enables serde support for the `ProblemDetails` struct.

mod problem_details;
mod problem_type;

pub use problem_details::*;
pub use problem_type::*;

// Serde related extensions for http
#[cfg(feature = "serde")]
mod serde;
