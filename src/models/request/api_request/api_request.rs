use super::RequestClientId;

pub struct ApiRequestBody<'a> {
    client_id: RequestClientId<'a>,
    client_version: ApiRequestClientVersion<'a>,
    tag_buf: u8,
}

pub struct ApiRequestClientVersion<'a> {
    length: u8,
    contents: &'a [u8],
}
