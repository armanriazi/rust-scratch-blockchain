use super::*;

///sha-256 means generate 32 byte hash
pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;


    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
