use std::{io, net::IpAddr};
use hyper::{body::Body, Request};
use tokio::io::{AsyncReadExt, Result};

struct Blacklist {
    addresses: Vec<IpAddr>,
}

impl Blacklist {
    fn new() -> Self {
        // Initialize Blacklist with an empty vector of addresses
        Blacklist {
            addresses: Vec::new(),
        }
    }
}



pub async fn request(stream: &mut tokio::net::TcpStream) -> io::Result<String> {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?; // Read from the TCP stream asynchronously
    let client_ip= stream.peer_addr();
    
    let request = String::from_utf8_lossy(&buf[..n]).to_string();

    // Do something with the request...
    println!("{:?}",client_ip);

    Ok(request) // Return the request string
}