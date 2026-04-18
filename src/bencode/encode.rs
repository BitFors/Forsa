//TODO

use super::Value;
use bytes::Bytes;
use std::collections::BTreeMap;

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
            Value::Bytes(b) => self.encode_bytes(b),
            Value::List(l) => self.encode_list(l),
            Value::Dict(d) => self.encode_dict(d),
        }
    }

    fn encode_int(&mut self, n: i64) {
        self.buf.push(b'i');
        self.push_int(n);
        self.buf.push(b'e');
    }

    fn encode_bytes(&mut self, b: &[u8]) {
        self.push_uint(b.len() as u64);
        self.buf.push(b':');
        self.buf.extend_from_slice(b);
    }

    fn encode_list(&mut self, items: &[Value]) {
        self.buf.push(b'l');
        for item in items {
            self.encode(item);
        }
        self.buf.push(b'e');
    }

    fn encode_dict(&mut self, map: &BTreeMap<Bytes, Value>) {
        self.buf.push(b'd');
        for (key, value) in map {
            self.encode_bytes(key);
            self.encode(value);
        }
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

    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }
}
