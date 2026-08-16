//! Reusable client and wire types for the eToro Public API.
//!
//! The README is included below so its `rust` snippets are compiled by
//! `cargo test --doc` and cannot silently drift from the real API.
#![doc = include_str!("../README.md")]

// Absolute paths emitted by cargo typify refer back to this crate by name.
extern crate self as etoro_agent;
pub mod client;
pub mod types;
