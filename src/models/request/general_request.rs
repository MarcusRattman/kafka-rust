use super::ApiRequestBody;
use super::IBeBytable;
use super::RequestBodytype;
use super::RequestClientId;

#[derive(Debug)]
pub struct Request {
    message_size: u32,
    request_header: RequestHeader,
    request_body: RequestBodytype,
}

impl Request {
    fn parse_body(header: &RequestHeader, body_bytes: &[u8]) -> Option<RequestBodytype> {
        match header.api_key {
            18 => Some(RequestBodytype::ApiRequest(ApiRequestBody::new(body_bytes))),
            _ => None,
        }
    }

    pub fn new(bytes: &[u8]) -> Self {
        let message_size = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let request_header = RequestHeader::new(&bytes[4..]);
        let body_index_start = 4 + request_header.length;
        let request_body = &bytes[body_index_start as usize..];
        let request_body = Self::parse_body(&request_header, request_body).unwrap();

        Self {
            message_size,
            request_header,
            request_body,
        }
    }

    pub fn api_key(&self) -> u16 {
        self.request_header.api_key
    }

    pub fn api_version(&self) -> u16 {
        self.request_header.api_version
    }

    pub fn correlation_id(&self) -> u32 {
        self.request_header.correlation_id
    }

    pub fn body(&self) -> &RequestBodytype {
        &self.request_body
    }
}

#[derive(Debug)]
struct RequestHeader {
    length: u16,
    api_key: u16,
    api_version: u16,
    correlation_id: u32,
    client_id: RequestClientId,
    tag_buf: u8,
}

impl IBeBytable for RequestHeader {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.api_key
            .to_be_bytes()
            .into_iter()
            .chain(self.api_version.to_be_bytes())
            .chain(self.correlation_id.to_be_bytes())
            .chain(self.client_id.to_be_bytes())
            .chain(self.tag_buf.to_be_bytes())
            .collect()
    }
}

impl RequestHeader {
    pub fn new(bytes: &[u8]) -> Self {
        let api_key = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
        let api_version = u16::from_be_bytes(bytes[2..4].try_into().unwrap());
        let correlation_id = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let client_id = RequestClientId::new(&bytes[8..]);

        // length of header itself which is 2 + 2 + 4 + 1 = 9
        // plus
        // length of client_id.contents + 2, since client_id.length is 2 bytes long
        let length = client_id.length + 11;

        Self {
            api_key,
            api_version,
            correlation_id,
            client_id,
            tag_buf: 0,
            length,
        }
    }
}
