#![allow(unused_imports)]
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Logs from your program will appear here!");
    for stream in listener.incoming() {
        match stream{
            Ok(mut stream) => {
                let mut buf= [0;512];
                loop {
                    let read_count = stream.read(&mut buf).unwrap();
                    if read_count == 0{
                        break;
                    }
                    stream.write(b"+PONG\r\n").unwrap();   
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
