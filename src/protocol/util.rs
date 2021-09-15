pub fn slice_as_u32(array: &[u8]) -> u32 {
    (u32::from(array[0]) << 24)
        + (u32::from(array[1]) << 16)
        + (u32::from(array[2]) << 8)
        + (u32::from(array[3]) << 0)
}

pub fn u32_as_slice(val: u32) -> [u8; 4] {
    [
        (val >> 24) as u8,
        (val >> 16) as u8,
        (val >> 8) as u8,
        (val >> 0) as u8,
    ]
}

pub fn slice_as_u64(array: &[u8]) -> u64 {
    (u64::from(array[0]) << 56)
        + (u64::from(array[1]) << 48)
        + (u64::from(array[2]) << 40)
        + (u64::from(array[3]) << 32)
        + (u64::from(array[4]) << 24)
        + (u64::from(array[5]) << 16)
        + (u64::from(array[6]) << 8)
        + (u64::from(array[7]) << 0)
}

pub fn u64_as_slice(val: u64) -> [u8; 8] {
    [
        (val >> 56) as u8,
        (val >> 48) as u8,
        (val >> 40) as u8,
        (val >> 32) as u8,
        (val >> 24) as u8,
        (val >> 16) as u8,
        (val >> 8) as u8,
        (val >> 0) as u8,
    ]
}

pub fn slice_as_u128(array: &[u8]) -> u128 {
    (u128::from(array[0]) << 120)
        + (u128::from(array[1]) << 112)
        + (u128::from(array[2]) << 104)
        + (u128::from(array[3]) << 96)
        + (u128::from(array[4]) << 88)
        + (u128::from(array[5]) << 80)
        + (u128::from(array[6]) << 72)
        + (u128::from(array[7]) << 64)
        + (u128::from(array[8]) << 56)
        + (u128::from(array[9]) << 48)
        + (u128::from(array[10]) << 40)
        + (u128::from(array[11]) << 32)
        + (u128::from(array[12]) << 24)
        + (u128::from(array[13]) << 16)
        + (u128::from(array[14]) << 8)
        + (u128::from(array[15]) << 0)
}

pub fn u128_as_slice(val: u128) -> [u8; 16] {
    [
        (val >> 120) as u8,
        (val >> 112) as u8,
        (val >> 104) as u8,
        (val >> 96) as u8,
        (val >> 88) as u8,
        (val >> 80) as u8,
        (val >> 72) as u8,
        (val >> 64) as u8,
        (val >> 56) as u8,
        (val >> 48) as u8,
        (val >> 40) as u8,
        (val >> 32) as u8,
        (val >> 24) as u8,
        (val >> 16) as u8,
        (val >> 8) as u8,
        (val >> 0) as u8,
    ]
}
