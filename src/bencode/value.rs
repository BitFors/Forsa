use bytes::Bytes;
use indexmap::IndexMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i64), // Technically there's no upper limit, i reckon it should fit in 64bits but I'd imagine this'll change in the future..
    Bytes(Bytes),
    List(Vec<Self>),
    Dict(IndexMap<Bytes, Self>),
}

impl Value {
    /// Create a dict from key-value pairs
    pub fn dict<I, K, V>(items: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<Bytes>,
        V: Into<Self>,
    {
        let mut map: IndexMap<Bytes, Self> = items
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        map.sort_keys();
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

    //TODO Type checks

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
    pub fn as_dict(&self) -> Option<&IndexMap<Bytes, Self>> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Get as mut dict
    pub fn as_dict_mut(&mut self) -> Option<&mut IndexMap<Bytes, Self>> {
        match self {
            Self::Dict(d) => Some(d),
            _ => None,
        }
    }

    //TODO Helpers
    //TODO From implementations
}
