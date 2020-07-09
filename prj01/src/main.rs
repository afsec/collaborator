#![warn(clippy::all)]

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

use chrono::{DateTime, Utc};
use std::time::Instant;

const MAX_BYTES: usize = 10_485_760; // 10 MBytes
                                     // const MAX_BYTES: usize = 20; // 10 MBytes

#[async_std::main]
async fn main() -> io::Result<()> {
    // Open up a TCP connection and create a URL.
    let listener = TcpListener::bind(("0.0.0.0", 8000)).await?;
    let addr = format!("http://{}", listener.local_addr()?);
    println!(
        "Listening on {addr}. (Request size =< {max_size} MBytes)",
        addr = addr,
        max_size = (MAX_BYTES / 1024) / 1024
    );

    // For each incoming TCP connection, spawn a task and call `accept`.
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        task::spawn(async {
            if let Err(err) = accept(stream).await {
                eprintln!("{}", err);
            }
        });
    }
    Ok(())
}

// Take a TCP stream, and convert it into sequential HTTP request / response pairs.
async fn accept(mut stream: TcpStream) -> io::Result<()> {
    let start = Instant::now();
    let now: DateTime<Utc> = Utc::now();
    let mut buf = vec![0u8; MAX_BYTES];
    let recv_length = stream.read(&mut buf).await?;
    let mut recv_data = String::new();

    for c in buf {
        if c > 0 {
            recv_data.push(c as char);
        }
    }

    let body = format!(
        r#"<!DOCTYPE html>
<html>
    <head>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8">
        <title>Directory listing for /</title>
    </head>
    <body>
        <h1>Directory listing for /</h1>
        <hr>
        <p>{now}</p>
    </body>
</html>"#,
        now = now
    );
    let proto_status = "HTTP/1.0 200 OK\r\n".to_string();
    let headers = format!("Server: nginx\r\nDate: {now}\r\nContent-type: text/html; charset=utf-8\r\nContent-Length: {body_len}\r\n\r\n", now = now,body_len = body.len());
    let response = format!(
        "{proto_status}{headers}{body}",
        proto_status = proto_status,
        headers = headers,
        body = body
    );

    stream.write_all(response.as_bytes()).await?;
    let duration = start.elapsed();

    println!(
        "-------[ {now}: {remote} -> {local} ({bytes_received} Bytes - elapsed {elapsed:?}) ]-------",
        now=now.to_rfc3339(),
        remote=stream.peer_addr()?,
        local=stream.local_addr()?,
        bytes_received=recv_length,
        elapsed=duration,
    );
    // TODO: Create Struct and Store recv_data into File or Database
    println!("{}", recv_data);
    // println!("{}", response);
    Ok(())
}
