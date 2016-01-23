extern crate rshttp;

use rshttp::HttpRequest;
use std::net::{TcpListener, TcpStream};
use std::io::BufReader;
use std::io::prelude::*;
use std::thread;
use std::str;


fn handle_client(stream: TcpStream) {
    let buf_stream = BufReader::new(stream);
    let mut request_str = String::new();
    for line_res in buf_stream.lines() {
        let line = line_res.unwrap();
        if line.is_empty() {
            break;
        }
        request_str = request_str + &(line + "\r\n");
    }
    let request = HttpRequest::new(&request_str).unwrap();
    println!("{:?}", request);
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => {
                panic!(e);
            }
        }
    }
    // close the socket server
    drop(listener);
}
