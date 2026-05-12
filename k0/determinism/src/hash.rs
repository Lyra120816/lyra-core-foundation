const FNV128_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
const FNV128_PRIME: u128 = 0x0000000001000000000000000000013b;

pub fn fnv1a128(bytes: &[u8]) -> u128 {
    let mut hash = FNV128_OFFSET;
    for byte in bytes {
        hash ^= u128::from(*byte);
        hash = hash.wrapping_mul(FNV128_PRIME);
    }
    hash
}

pub fn fnv1a128_hex(bytes: &[u8]) -> String {
    format!("{:032x}", fnv1a128(bytes))
}

pub fn stable_hash_label(label: &str, text: &str) -> String {
    let mut bytes = Vec::with_capacity(label.len() + 1 + text.len());
    bytes.extend_from_slice(label.as_bytes());
    bytes.push(0x00);
    bytes.extend_from_slice(text.as_bytes());
    format!("fnv1a128:{}", fnv1a128_hex(&bytes))
}
