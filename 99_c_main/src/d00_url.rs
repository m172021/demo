use super::*;

#[derive(Clone)]
pub struct Url {
    pub(crate) target_url: String,
}

impl Url {
    pub fn to_256(&self) -> Url256 {
        // phind.com
        use digest::Digest;
        use sha2::Sha256;
        let mut hasher = Sha256::new();
        hasher.update(&self.target_url);
        let result = hasher.finalize();
        let out: [u8; 32] = result.try_into().expect("incorrect length");
        Url256 { hash: out }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Url256 {
    hash: [u8; 32], // Birthday Paradox: For 2^64 urls, Pr collision < 2^(-128)
}

pub struct ShardGroup<const N: usize> {
    data: [u8; N],
}
