use std::io::{Read, Write};
use std::net::TcpStream;

use crate::models::{Request, Response};

pub fn read_respond(mut tcp_stream: TcpStream) {
    let mut buf = [0; 24];
    tcp_stream.read(&mut buf).unwrap();

    let req = Request::new(buf);
    println!("Received: {:?}", req);

    let res = Response::from(req);

    tcp_stream.write_all(&res.to_be_bytes()).unwrap();
    println!("Sent: {:?}", res);
    tcp_stream.shutdown(std::net::Shutdown::Both).unwrap();
}
