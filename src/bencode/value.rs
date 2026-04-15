use bytes::Bytes;
use indexmap::IndexMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Integer(i64), // Technically there's no upper limit, i reckon it should fit in 64bits but I'd imagine this'll change in the future..
    Bytes(Vec<u8>),
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
        Value::List(items.into_iter().map(Into::into).collect())
    }
}
