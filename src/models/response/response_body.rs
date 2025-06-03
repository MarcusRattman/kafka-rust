use crate::models::Request;

#[derive(Debug)]
pub struct ResponseBody {
    response: i16,
}

impl ResponseBody {
    pub fn new(req: &Request) -> Self {
        Self {
            response: match_api(req),
        }
    }

    pub fn to_be_bytes(&self) -> [u8; size_of::<Self>()] {
        self.response.to_be_bytes()
    }
}

fn match_api(req: &Request) -> i16 {
    let key = req.header.request_api_key;
    let version = req.header.request_api_version;

    match (key, version) {
        _ => 35,
    }
}
