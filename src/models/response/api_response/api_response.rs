use super::api_version::ApiVersionsArray;
use crate::models::traits::IBeBytable;

pub struct ApiResponse {
    message_size: u32,
    correlation_id: u32,
    api_response_body: ApiResponseBody,
}

pub struct ApiResponseBody {
    error_code: u16,
    api_versions: ApiVersionsArray,
    throttle: u32,
    tag_buf: u8,
}

impl IBeBytable for ApiResponse {
    fn to_be_bytes(&self) -> Vec<u8> {
        self.message_size
            .to_be_bytes()
            .into_iter()
            .chain(self.correlation_id.to_be_bytes())
            .chain(self.api_response_body.to_be_bytes())
            .collect()
    }
}

impl IBeBytable for ApiResponseBody {
    fn to_be_bytes(&self) -> Vec<u8> {
        let err = &self.error_code.to_be_bytes();
        let api_ver = self.api_versions.to_be_bytes();
        let throttle = &self.throttle.to_be_bytes();
        let tag_buf = &self.tag_buf.to_be_bytes();

        [err, api_ver.as_slice(), throttle, tag_buf].concat()
    }
}
