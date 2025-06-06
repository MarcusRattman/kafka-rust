use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

use crate::models::Request;

pub fn read_respond(mut tcp_stream: TcpStream) {
    let buf = &mut [];
    tcp_stream.read(buf).unwrap();

    // let req = ResponseBuilder::parse_request(buf);
    // println!("Received: {:?}", req);

    // let res = ResponseBuilder::from(req);

    // tcp_stream.write_all(&res.to_be_bytes()).unwrap();
    // println!("Sent: {:?}", res);
    tcp_stream.shutdown(Shutdown::Both).unwrap();
}
