//TODO

use super::Value;

pub fn encode(value: &Value) -> Vec<u8> {
    //TODO Might change after impl
    let mut encoder = Encoder::new();
    encoder.encode(value);
    encoder.into_bytes()
}

pub struct Encoder {
    buf: Vec<u8>, //TODO might change after impl
}

impl Encoder {}
