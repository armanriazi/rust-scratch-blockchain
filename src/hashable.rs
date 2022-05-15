use super::*;

pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    //This exmp use sha-1 means 32 byte
    fn hash (&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
    }
}
