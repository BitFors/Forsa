//TODO

use super::Value;

pub fn encode(value: &Value) -> Vec<u8> {
    //TODO Might change after impl
    let mut encoder = Encoder::new();
    encoder.encode(value);
    encoder.into_bytes()
}
/// Streaming encoder
pub struct Encoder {
    buf: Vec<u8>, //TODO might change after impl
}

impl Encoder {
    /// Create new encoder.
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Create with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
        }
    }

    /// Encode a value
    pub fn encode(&mut self, value: &Value) {
        match value {
            Value::Integer(n) => self.encode_int(*n),
        }
    }

    fn encode_int(&mut self, n: i64) {
        self.buf.push(b'i');
        self.push_int(n);
        self.buf.push(b'e');
    }

    fn push_uint(&mut self, mut n: u64) {
        if n == 0 {
            self.buf.push(b'0');
            return;
        }
        let mut tmp = [0u8; 20];
        let mut pos = 20;
        while n > 0 {
            pos -= 1;
            tmp[pos] = b'0' + (n % 10) as u8;
            n /= 10;
        }
        self.buf.extend_from_slice(&tmp[pos..]);
    }

    fn push_int(&mut self, n: i64) {
        if n < 0 {
            self.buf.push(b'-');
            self.push_uint(n.unsigned_abs());
        } else {
            self.push_uint(n as u64);
        }
    }
}
