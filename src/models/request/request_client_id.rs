use super::IBeBytable;

#[derive(Debug)]
pub struct RequestClientId {
    pub length: u16,
    contents: Vec<u8>,
}

impl RequestClientId {
    pub fn new(bytes: &[u8]) -> Self {
        let length = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
        let contents = bytes[2..(2 + length as usize)].to_vec();

        Self { length, contents }
    }
}

impl IBeBytable for RequestClientId {
    fn to_be_bytes(&self) -> Vec<u8> {
        [&self.length.to_be_bytes(), self.contents.as_slice()].concat()
    }
}
