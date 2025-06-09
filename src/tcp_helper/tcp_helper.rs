use std::io::Read;
use std::net::{Shutdown, TcpStream};

use crate::models::Request;

pub fn read_respond(mut tcp_stream: TcpStream) {
    let mut buf = vec![0; 1024];
    let bytes_read = tcp_stream.read(&mut buf).unwrap();
    buf.truncate(bytes_read);

    // let req = ResponseBuilder::parse_request(buf);
    // println!("Received: {:?}", req);

    let req = Request::new(&buf);
    println!("{:?}", req);

    // let res = ResponseBuilder::from(req);

    // tcp_stream.write_all(&res.to_be_bytes()).unwrap();
    // println!("Sent: {:?}", res);
    tcp_stream.shutdown(Shutdown::Both).unwrap();
}
