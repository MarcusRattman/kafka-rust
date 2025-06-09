use super::IBeBytable;

#[derive(Debug)]
pub struct BodyClientId {
    pub length: u8,
    contents: Vec<u8>,
}

impl BodyClientId {
    pub fn new(bytes: &[u8]) -> Self {
        let length = u8::from_be_bytes([bytes[0]]);
        let contents = bytes[1..(length as usize)].to_vec();

        Self { length, contents }
    }
}

impl IBeBytable for BodyClientId {
    fn to_be_bytes(&self) -> Vec<u8> {
        [&self.length.to_be_bytes(), self.contents.as_slice()].concat()
    }
}
