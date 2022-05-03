use sha1::{Digest, Sha1};

pub fn full_hash(bytes: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:x?}", b))
        .collect::<Vec<String>>()
        .join("")
}
