//! # mini_rest
//! Este crate provee un servidor HTTP básico con funciones para manejar direcciones, iniciar servidores, y más.
//!
//! ## Ejemplo
//! ```rust
//! use mini_rest::server::{self, ServerActions};
//!
//! let addr = String::from("127.0.0.1:8080");
//! let mut server = server::new(addr);
//!
//! server.start();
//! ```

pub mod server;
