use std::io::{BufRead, BufReader, Result, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use tokio::task::JoinHandle;

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

pub async fn handle_connection(server_address: SocketAddr, run_before: fn(SocketAddr)) -> Result<()> {
    let listener = TcpListener::bind(server_address)?;

    let http_server: JoinHandle<Result<()>> = tokio::spawn(async move {
        for stream in listener.incoming() {
            let stream = stream?;

            println!("Connection Established!");

            handle_client(stream);
        }

        Ok(())
    });

    run_before(server_address);

    http_server.await.expect("Server thread failed")?;
    return Ok(());
}
