//! # mini_rest
//!This crate provides a basic HTTP server with functions for handling addresses, starting servers, and more.
//!
//! ## Example of use
//! ```rust,no_run
//! use mini_rest::server::{self, ServerActions};
//! #[tokio::main]
//! async fn main() {
//!     let mut server = server::new("127.0.0.1:8080".to_string());
//!     server.start().await;
//! }
//! ```

pub mod server;

// TODO: DOCUMENT THIS
#[tokio::test]
async fn test_server() {
    use server::{self, ServerActions};
    // Create a new server
    let mut server = server::new("127.0.0.1:8080".to_string());
    server.add_route("/", || {
        //println!("New handler");
    });
    // Start listening
    server.start().await;
}
