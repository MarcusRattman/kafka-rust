pub trait IBeBytable {
    fn to_be_bytes(&self) -> Vec<u8>;
}
