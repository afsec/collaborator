#![warn(clippy::all)]

// use std::str::FromStr;
use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

const MAX_BYTES: usize = 10_485_760; // 10 MBytes
// const MAX_BYTES: usize = 20; // 10 MBytes

#[async_std::main]
async fn main() -> io::Result<()> {
    // Open up a TCP connection and create a URL.
    let listener = TcpListener::bind(("0.0.0.0", 8000)).await?;
    let addr = format!("http://{}", listener.local_addr()?);
    println!("listening on {}", addr);

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
    // let mut stream = stream.clone();
    let mut recv = vec![0u8; MAX_BYTES];
    let recv_length = stream.read_to_end(&mut recv).await?;
    let mut data = String::new();
    for c in recv {
        if c > 1 {
            data.push(c as char);
        }
    }
    // dbg!(recv);
    // let raw_http_request = String::from_utf8(recv);
    println!("starting new connection from {}", stream.peer_addr()?);
    println!("Bytes received: {}", recv_length);
    println!("{}",data);
    // dbg!(raw_http_request);
    Ok(())
}
