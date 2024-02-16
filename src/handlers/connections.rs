pub async fn handle_connection(stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {
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