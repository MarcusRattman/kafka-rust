use super::RequestClientId;

pub struct Request<'a> {
    message_size: u32,
    request_header: RequestHeader<'a>,
    request_body: &'a [u8],
}

struct RequestHeader<'a> {
    api_key: u16,
    api_version: u16,
    correlation_id: u32,
    client_id: RequestClientId<'a>,
    tag_buf: u8,
}

impl<'a> RequestHeader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let api_key = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
        let api_version = u16::from_be_bytes(bytes[2..4].try_into().unwrap());
        let correlation_id = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        let client_id = RequestClientId::new(&bytes[8..]);

        Self {
            api_key,
            api_version,
            correlation_id,
            client_id,
            tag_buf: 0,
        }
    }
}
