use bytes::Bytes;

use super::request_header::ReqHeader;
use crate::models::request::request_body::ReqBody;

pub struct Request {
    pub size: i32,
    pub header: ReqHeader,
    pub body: ReqBody,
}

impl Request {
    pub fn new(bytes: [u8; 24]) -> Self {
        let size = i32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let header = ReqHeader::new(bytes[4..12].try_into().unwrap());
        let body = ReqBody {};

        Self { size, header, body }
    }
}
