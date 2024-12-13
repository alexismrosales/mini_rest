use std::{future::Future, pin::Pin};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    address: String,
}

pub trait ServerActions {
    fn start(self) -> Pin<Box<dyn Future<Output = ()> + Send>>;
    fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn() + Send + Sync + 'static;
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
    /// let server = server::new("127.0.0.1:8080".to_string()); assert_eq!(server.ip(), "127.0.0.1");
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
    /// To use the `start` function, you need to initialize an async runtime, such as Tokio. Here's an example:
    /// ```rust,no_run
    /// use mini_rest::server::{self, ServerActions};
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut server = server::new("127.0.0.1:8080".to_string());
    ///     server.start().await;
    /// }
    /// ```
    fn start(self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            if let Err(e) = start_server(self).await {
                eprintln!("Error: {}", e);
            }
        })
    }
    fn add_route<F>(&mut self, path: &str, _handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        // #TOREMOVE
        println!("Test path: {}", path);
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
/// A `Server` instance that supports main actions like starting the server or retrieving its details.
pub fn new(addr: String) -> Server {
    Server { address: addr }
}

/// Starts the HTTP server asynchronously.
///
/// This function performs the actual logic for starting the server.
/// It is separate from the trait `ServerActions` to avoid conflicts and allow more flexibility.
async fn start_server(server: Server) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(server.address.clone()).await.unwrap();
    println!("Starting listening at {}...", server.address);
    loop {
        // Wait until accept a new petition from a new client
        match listener.accept().await {
            Ok((socket, _)) => {
                // Proccess connections concurrently
                tokio::spawn(async move {
                    handle_client(socket).await;
                });
            }
            Err(e) => println!("Error in acception: {}", e),
        }
    }
}

async fn handle_client(mut socket: TcpStream) {
    println!(
        "New client connected, Remote addr {:?}",
        socket.peer_addr().unwrap()
    );
    socket.set_nodelay(true).unwrap();
    let mut buffer = [0; 2048];
    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(size) => {
                let header = String::from_utf8_lossy(&buffer[..size]);
                println!("Received data from client: {}", header);
            }
            Err(e) => {
                println!("Error reading from socket: {:?}", e);
                break;
            }
        }
    }
}
