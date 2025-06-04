pub struct ApiRequest {
    message_size: u32,
    request_header: ApiRequestHeader,
    request_body: ApiRequestBody,
}

pub struct ApiRequestHeader {
    api_key: u16,
    api_version: u16,
    correlation_id: u32,
    client_id: ApiRequestClient,
    tag_buf: u8,
}

pub struct ApiRequestBody {
    client_id: ApiRequestClientId,
    client_version: ApiRequestClientVersion,
    tag_buf: u8,
}

pub struct ApiRequestClientId {
    length: u16,
    contents: Vec<u8>,
}

pub struct ApiRequestClientVersion {
    length: u8,
    contents: Vec<u8>,
}
