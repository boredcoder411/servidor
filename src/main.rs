use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::env;

fn handle_client(mut stream: TcpStream, file_path: &str) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    let contents = fs::read_to_string(file_path).unwrap();
    let response = format!("{}{}", response, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (addr, file_path) = match args.get(1) {
        Some(port_str) => {
            let port = port_str.parse::<u16>().unwrap();
            let addr = format!("127.0.0.1:{}", port);
            let file_path = match args.get(2) {
                Some(path) => path,
                None => {
                    eprintln!("Usage: servidor <port> <file_path>");
                    return;
                }
            };
            (addr, file_path)
        }
        None => {
            eprintln!("Usage: servidor <port> <file_path>");
            return;
        }
    };

    let listener = TcpListener::bind(&addr).unwrap();

    println!("Server listening on {}", addr);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_client(stream, file_path);
    }
}
