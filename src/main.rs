#![allow(unused_imports)]
use std::{io::Write, net::TcpListener};

mod models;
mod tcp_helper;
use models::Response;

use tcp_helper::read_respond;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                read_respond(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
