use std::io::{ErrorKind, Result};
use std::net::SocketAddr;

use clap::Parser;

use crate::cli::*;
use crate::tcp::handle_connection;

mod cli;
mod tcp;

fn callback(addr: SocketAddr) {
    println!("Connected to http://{addr}");
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Args = Parser::parse();

    if let Err(e) = handle_connection(SocketAddr::from((args.addr, args.port)), callback).await {
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

    Ok(())
}
