use super::api_version::ApiVersion;

pub struct ApiResponse {
    message_size: u32,
    correlation_id: u32,
    api_response_body: ApiResponseBody,
}

pub struct ApiResponseBody {
    error_code: u16,
    api_versions: Vec<ApiVersion>,
    throttle: u32,
    tag_buf: u8,
}
