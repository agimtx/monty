//! Core implementation for the `aq` module exposed inside Monty.
//!
//! This crate owns the module's domain behavior. The `monty` crate keeps only
//! the VM adapter that turns these Rust values into Monty `Value`s.

/// Returns the greeting exposed as `aq.hello()`.
pub fn hello() -> &'static str {
    "hello"
}
