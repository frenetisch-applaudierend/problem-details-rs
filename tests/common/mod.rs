//! Shared helpers for the serialization tests.
//!
//! This module is compiled into every integration test binary that declares
//! `mod common;`, so not every item is used by every binary.
#![allow(dead_code)]

pub mod assert;
pub mod cases;

#[cfg(feature = "xml")]
pub mod xml_canon;
