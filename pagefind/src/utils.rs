use sha1::{Digest, Sha1};

/// Symbols that count as part of a word
/// (specifically, the "Punctuation, Connector" Unicode category)
pub const WORD_SYMBOLS: [char; 10] = ['_', '‿', '⁀', '⁔', '︳', '︴', '﹍', '﹎', '﹏', '＿'];

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
