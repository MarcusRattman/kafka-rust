use super::BodyClientId;

#[derive(Debug)]
pub struct ApiRequestBody {
    client_id: BodyClientId, // Supposed to be a compact string
    client_version: ApiRequestClientVersion,
    tag_buf: u8,
}

impl ApiRequestBody {
    pub fn new(body_bytes: &[u8]) -> Self {
        let client_id = BodyClientId::new(body_bytes);
        let clid_len = client_id.length as usize;
        let client_version = ApiRequestClientVersion::new(&body_bytes[(clid_len as usize)..]);

        Self {
            client_id,
            client_version,
            tag_buf: 0,
        }
    }
}

#[derive(Debug)]
pub struct ApiRequestClientVersion {
    length: u8,
    contents: Vec<u8>,
}

impl ApiRequestClientVersion {
    pub fn new(bytes: &[u8]) -> Self {
        let length = u8::from_be_bytes([bytes[0]]);
        let contents = bytes[1..(length as usize)].to_vec();

        Self { length, contents }
    }
}
