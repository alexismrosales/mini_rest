//! # mini_rest
//!This crate provides a basic HTTP server with functions for handling addresses, starting servers, and more.
//!
//! ## Example of use
//! ```rust,no_run
//! use mini_rest::server::{self, ServerActions};
//!
//! let addr = String::from("127.0.0.1:8080");
//! let mut server = server::new(addr);
//!
//! ```

pub mod server;
