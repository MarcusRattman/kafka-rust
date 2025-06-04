use super::ResponseBody;
use crate::models::Request;

#[derive(Debug)]
pub struct Response {
    size: u32,
    correlation_id: u32,
    body: ResponseBody,
}

impl Response {
    pub fn to_be_bytes(&self) -> Vec<u8> {
        [
            self.size.to_be_bytes().as_slice(),
            self.correlation_id.to_be_bytes().as_slice(),
            self.body.to_be_bytes().as_slice(),
        ]
        .concat()
    }
}

impl From<Request> for Response {
    fn from(req: Request) -> Self {
        Self {
            size: size_of::<Response>() as u32,
            correlation_id: req.header.correlation_id,
            body: ResponseBody::new(&req),
        }
    }
}
