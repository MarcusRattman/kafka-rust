use super::ApiRequestBody;

#[derive(Debug)]
pub enum RequestBodytype {
    ApiRequest(ApiRequestBody),
}
