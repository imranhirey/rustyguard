use std::net::SocketAddr;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener, stream};

pub async fn setup_and_listen() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([192, 168, 0, 24], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let client_ip: SocketAddr = addr;
        println!("Client connected from {}", client_ip);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // Parse the HTTP request
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;

    let request = String::from_utf8_lossy(&buf[..n]);
    let response = if request.starts_with("GET / HTTP/1.1\r\n") {
        "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello, world!"
    } else {
        "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found"
    };

    // Write the response to the stream
    stream.write_all(response.as_bytes()).await?;
    Ok(())
    
}

fn format_http_request(request: String) -> String {
    let lines: Vec<&str> = request.split("\r\n").collect();
    let formatted_lines: Vec<String> = lines.iter()
        .map(|line| format!("    {}", line))
        .collect();

    formatted_lines.join("\n")
}


