use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};

use clap::Parser;

use crate::cli::*;

mod cli;

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    

    println!("Request: {http_request:#?}");

    stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_connection(server_address: SocketAddr) -> Result<(), io::Error> {
    let listener = TcpListener::bind(server_address)?;

    println!("Listening on {server_address}");

    for stream in listener.incoming() {
        let stream = stream?;

        println!("Connection Established!");

        handle_client(stream);
    }

    return Ok(());
}

fn main() {
    let args: Args = Parser::parse();
    if let Err(e) = handle_connection(SocketAddr::from((args.addr, args.port))) {
        match e.kind() {
            ErrorKind::AddrInUse => {
                eprintln!(
                    "Server ip address or port not available for address: {:?}:{}",
                    args.addr, args.port
                );
            }
            ErrorKind::WouldBlock => {
                eprintln!("Requested operation is blocking");
            }
            ErrorKind::OutOfMemory => {
                eprintln!("Out of Memory, WHAT ARE YOU DOING");
            }
            _ => {
                eprintln!("ERROR: could not run server because of {e}")
            }
        }
    }
}
