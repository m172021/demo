use super::*;

#[derive(Clone)]
pub struct Url {
    pub(crate) target_url: String,
}

impl Url {
    pub fn to_256(&self) -> Url256 {
        todo!("call sha 256")
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Url256 {
    hash: [u8; 256], // Birthday Paradox: For 2^64 urls, Pr collision < 2^(-128)
}

pub struct ShardGroup<const N: usize> {
    data: [u8; N],
}
