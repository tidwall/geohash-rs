const BASE32_ENCODING: [u8; 32] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'b', b'c', b'd', b'e', b'f', b'g',
    b'h', b'j', b'k', b'm', b'n', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];
const BASE32_DECODING: [u8; 128] = [
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0',
    b'0', b'0', 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, b'0', 0x11, 0x12, b'0', 0x13, 0x14, b'0',
    0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, b'0', b'0', b'0', b'0', b'0',
];

pub fn encode(mut x: u64) -> [u8; 12] {
    let mut bytes = [0u8; 12];
    for i in 0..12 {
        bytes[11 - i] = BASE32_ENCODING[x as usize & 0x1f];
        x >>= 5;
    }
    bytes
}

pub fn decode(s: &[u8]) -> u64 {
    let mut x = 0;
    for b in s {
        x = (x << 5) | BASE32_DECODING[*b as usize] as u64
    }
    x
}

pub fn valid_byte(b: u8) -> bool {
    b < 128 && (b == b'a' || BASE32_DECODING[b as usize] != 0)
}
