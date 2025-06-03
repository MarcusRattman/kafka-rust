#[derive(Debug)]
pub struct ReqHeader {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    // client_id: Option<String>,
    // tag_buffer: [u8; 4], //TODO find an appropriate size or whatever
}

impl ReqHeader {
    /**
     * ### Accepts an array of 8 bytes.
     * 0..2 -> api key
     * 2..4 -> api version
     * 4..8 -> correlation id
     */
    pub fn new(bytes: [u8; 8]) -> Self {
        Self {
            request_api_key: i16::from_be_bytes(bytes[0..2].try_into().unwrap()),
            request_api_version: i16::from_be_bytes(bytes[2..4].try_into().unwrap()),
            correlation_id: i32::from_be_bytes(bytes[4..8].try_into().unwrap()),
        }
    }
}
