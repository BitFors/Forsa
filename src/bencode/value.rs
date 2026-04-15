use bytes::Bytes;
use indexmap::IndexMap;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BencodeValue {
    Integer(i64), // Technically there's no upper limit, in practice it should fit in 64bits.
    Bytes(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(IndexMap<Bytes, BencodeValue>),
}
