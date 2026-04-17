use bytes::Bytes;
use std::collections::BTreeMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i64), // Technically there's no upper limit, i reckon it should fit in 64bits but I'd imagine this'll change in the future..
    Bytes(Bytes),
    List(Vec<Self>),
    Dict(BTreeMap<Bytes, Self>),
}

impl Value {
    /// Create a dict from key-value pairs
    pub fn dict<I, K, V>(items: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Bytes>,
        V: Into<Self>,
    {
        let map: BTreeMap<Bytes, Self> = items
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self::Dict(map)
    }

    /// Create a list from an iterator of values
    pub fn list<I, V>(items: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<Value>,
    {
        Self::List(items.into_iter().map(Into::into).collect())
    }

    /// Check if int
    pub fn is_int(&self) -> bool {
        matches!(self, Self::Integer(_))
    }
    /// Check if bytes
    pub fn is_bytes(&self) -> bool {
        matches!(self, Self::Bytes(_))
    }
    /// Check if list
    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }
    /// Check if dict
    pub fn is_dict(&self) -> bool {
        matches!(self, Self::Dict(_))
    }

    //TODO Slice-based accessors? Maybe?

    /// Get as int
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as bytes
    pub fn as_bytes(&self) -> Option<&Bytes> {
        match self {
            Self::Bytes(b) => Some(b),
            _ => None,
        }
    }

    /// Get as UTF8 string
    pub fn as_str(&self) -> Option<&str> {
        self.as_bytes().and_then(|b| std::str::from_utf8(b).ok())
    }

    /// Get as list
    pub fn as_list(&self) -> Option<&Vec<Self>> {
        match self {
            Self::List(l) => Some(l),
            _ => None,
        }
    }

    /// Get as mut list
    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Self>> {
        match self {
            Self::List(l) => Some(l),
            _ => None,
        }
    }

    /// Get as dict
    pub fn as_dict(&self) -> Option<&BTreeMap<Bytes, Self>> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Get as mut dict
    pub fn as_dict_mut(&mut self) -> Option<&mut BTreeMap<Bytes, Self>> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Get value from dict by key
    pub fn get<K: AsRef<[u8]>>(&self, key: K) -> Option<&Self> {
        self.as_dict()?.get(key.as_ref())
    }

    /// Get int from dict
    pub fn get_int<K: AsRef<[u8]>>(&self, key: K) -> Option<i64> {
        self.get(key)?.as_int()
    }

    /// Get bytes from dict
    pub fn get_bytes<K: AsRef<[u8]>>(&self, key: K) -> Option<&Bytes> {
        self.get(key)?.as_bytes()
    }

    /// Get string from dict
    pub fn get_str<K: AsRef<[u8]>>(&self, key: K) -> Option<&str> {
        self.get(key)?.as_str()
    }

    /// Get list from dict
    pub fn get_list<K: AsRef<[u8]>>(&self, key: K) -> Option<&Vec<Self>> {
        self.get(key)?.as_list()
    }

    /// Get nested dict
    pub fn get_dict<K: AsRef<[u8]>>(&self, key: K) -> Option<&BTreeMap<Bytes, Self>> {
        self.get(key)?.as_dict()
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Integer(n)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Integer(n as i64)
    }
}
// Bencode does not support floats, need to add a warning if passing in floats.
impl From<u32> for Value {
    fn from(n: u32) -> Self {
        Value::Integer(n as i64)
    }
}
impl From<u64> for Value {
    fn from(n: u64) -> Self {
        Value::Integer(n as i64)
    }
}
impl From<usize> for Value {
    fn from(n: usize) -> Self {
        Value::Integer(n as i64)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::Bytes(Bytes::copy_from_slice(s.as_bytes()))
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Bytes(Bytes::from(s))
    }
}

impl From<&[u8]> for Value {
    fn from(b: &[u8]) -> Self {
        Value::Bytes(Bytes::copy_from_slice(b))
    }
}

impl From<Vec<u8>> for Value {
    fn from(b: Vec<u8>) -> Self {
        Value::Bytes(Bytes::from(b))
    }
}

impl From<Bytes> for Value {
    fn from(b: Bytes) -> Self {
        Value::Bytes(b)
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {
        Value::List(v.into_iter().map(Into::into).collect())
    }
}

impl From<BTreeMap<Bytes, Value>> for Value {
    fn from(mut m: BTreeMap<Bytes, Value>) -> Self {
        Value::Dict(m)
    }
}
