use std::net::SocketAddr;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpListener};
use crate::security::check;



pub async fn setup_and_listen() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let client_ip = stream.peer_addr()?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream) -> Result<(), Box<dyn std::error::Error>> {

    // Parse the HTTP request
 
    

    let request=check::request(&mut stream).await?;
    
    let response = if request.starts_with("GET / HTTP/1.1\r\n") {
        // Creating HTML content with client IP and request info
        let html_content = format!(
            "<!DOCTYPE html>\
            <html>\
            <head>\
                <title>Client Information</title>\
                <style>\
                    body {{ text-align: center; }}\
                    .container {{ height: 100vh; display: flex; flex-direction: column; justify-content: center; }}\
                    .message {{ font-size: 24px; }}\
                </style>\
            </head>\
            <body>\
                <div class=\"container\">\
                    <h1>Client Information</h1>\
                    <p class=\"message\">Your IP Address: {}</p>\
                    <p class=\"message\">Request: </p>\
                </div>\
            </body>\
            </html>",
             request
        );

        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", html_content.len(), html_content)
    } else {
        "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found".to_string()
    };

    // Write the response to the stream
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
