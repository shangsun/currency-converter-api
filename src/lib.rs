//! Library surface for the currency-converter-api.
//!
//! Exposing the modules as a library (in addition to the binary) lets
//! integration tests in `tests/` drive the handlers, state, and service
//! abstractions directly.

pub mod config;
pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod services;
pub mod state;
