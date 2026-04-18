pub fn mask_from_right(value: u64, bits: u64) -> u64 {
    value & u64::MAX >> bits
}
pub fn mask_from_left(value: u64, bits: u64) -> u64 {
    value & u64::MAX >> (64 - bits)
}
