use std::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
}

pub trait ServerActions {
    fn start(&mut self);
}

pub trait ServerInfo {
    /// Returns full address of the server.
    ///
    /// # Example
    /// ```
    /// use mini_rest::server::{self, ServerInfo};
    /// let server = server::new("127.0.0.1:8080".to_string());
    /// assert_eq!(server.address(), "127.0.0.1:8080");
    /// ```
    fn address(&self) -> &str;
    /// Returns the IP address of the server.
    ///
    /// # Example
    /// ```
    /// use mini_rest::server::{self, ServerInfo};
    /// let server = server::new("127.0.0.1:8080".to_string());
    /// assert_eq!(server.ip(), "127.0.0.1");
    /// ```
    fn ip(&self) -> &str;
    /// Returns the port where server is listening.
    ///
    /// # Example
    /// ```
    /// use mini_rest::server::{self, ServerInfo};
    /// let server = server::new("127.0.0.1:8080".to_string());
    /// assert_eq!(server.port(), 8080);
    fn port(&self) -> i32;
}

impl ServerActions for Server {
    fn start(&mut self) {
        if let Err(e) = start(self) {
            eprintln!("Failed to start server: {}", e);
        }
    }
}

impl ServerInfo for Server {
    fn address(&self) -> &str {
        &self.address
    }

    fn ip(&self) -> &str {
        let (ip, _) = self.address.split_once(':').unwrap();
        ip
    }

    fn port(&self) -> i32 {
        let (_, port) = self.address.split_once(':').unwrap();
        port.parse().unwrap()
    }
}

/// Start a new HTTP server
/// # Example
/// ```
/// // Create a server with a specific address
/// use mini_rest::server::{self, ServerInfo};
/// let server = server::new("192.168.1.253:8080".to_string());
/// ```
/// # Paramrests
/// - `address`: An optional address for the server.
/// # Returns
/// A `Server` struct that allows you to execute main actions like starting the server or adding routes.
pub fn new(addr: String) -> Server {
    Server { address: addr }
}

fn start(server: &mut Server) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(server.address.clone())?;
    println!("Starting server at {}...", server.address);
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(stream: TcpStream) {
    // ...
}
