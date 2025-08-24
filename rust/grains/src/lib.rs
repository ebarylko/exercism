fn is_valid_square(s: &u32) -> bool {
    (1..=64).contains(s)
}

pub fn square(s: u32) -> u64 {
    Some(s)
        .filter(is_valid_square)
        .map(|sq_num| 2u64.pow(sq_num - 1))
        .unwrap()
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
