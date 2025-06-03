use crate::models::Request;

#[derive(Debug)]
pub struct Response {
    size: i32,
    correlation_id: i32,
}

impl Response {
    pub fn new(size: i32, correlation_id: i32) -> Self {
        Self {
            size,
            correlation_id,
        }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        [self.size.to_be_bytes(), self.correlation_id.to_be_bytes()].concat()
    }
}

impl From<Request> for Response {
    fn from(req: Request) -> Self {
        Self {
            size: size_of::<Response>() as i32,
            correlation_id: req.header.correlation_id,
        }
    }
}
