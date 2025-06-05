use crate::models::traits::IBeBytable;

pub struct ApiVersionsArray {
    length: u8,
    api_versions: Vec<ApiVersion>,
}
pub struct ApiVersion {
    api_key: u16,
    min_sup: u16,
    max_sup: u16,
    tag_buf: u8,
}

impl IBeBytable for ApiVersionsArray {
    fn to_be_bytes(&self) -> Vec<u8> {
        let len = &(self.length + 1).to_be_bytes();
        let apis = self
            .api_versions
            .iter()
            .flat_map(|api| api.to_be_bytes())
            .collect::<Vec<u8>>();

        [len, apis.as_slice()].concat()
    }
}

impl IBeBytable for ApiVersion {
    fn to_be_bytes(&self) -> Vec<u8> {
        let key = &self.api_key.to_be_bytes();
        let min = &self.min_sup.to_be_bytes();
        let max = &self.max_sup.to_be_bytes();
        let tag = self.tag_buf.to_be_bytes();

        [key, min, max, tag.as_slice()].concat()
    }
}
