pub struct RequestClientId<'a> {
    length: u16,
    contents: &'a [u8],
}

impl<'a> RequestClientId<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        // Compact string has length of n + 1
        let length = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
        let contents = &bytes[2..(length as usize)];

        Self { length, contents }
    }
}
