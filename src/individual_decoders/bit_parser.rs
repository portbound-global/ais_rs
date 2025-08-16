pub struct BitField {
    pub start: usize,
    pub len: usize,
}

const STRING_CONVERSION_LOOKUP: [&str; 64] = [
    "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
    "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "*", "]", "*", "*", " ", "!", "", "*", "$", "%",
    "&", "*", "(", ")", "*", "*", ",", "-", ".", "/", "0", "1", "2", "3", "4", "5", "6", "7", "8",
    "9", ":", ";", "<", "=", ">", "?",
];

pub fn get<T: ValueFromBits>(bits_vec: &Vec<u8>, field: BitField) -> T {
    let slice = &bits_vec[field.start..field.start + field.len];
    T::from_bits(slice)
}

fn bit_to_usize_helper(chunk: &[u8]) -> usize {
    let mut value = 0usize;

    for &bit in chunk {
        value = (value << 1) | (bit as usize);
    }
    value
}

pub trait ValueFromBits: Sized {
    fn from_bits(bits: &[u8]) -> Self;
}

impl ValueFromBits for bool {
    fn from_bits(bits: &[u8]) -> Self {
        bit_to_usize_helper(bits) == 1
    }
}

impl ValueFromBits for u32 {
    fn from_bits(bits: &[u8]) -> Self {
        bit_to_usize_helper(bits) as u32
    }
}

impl ValueFromBits for u8 {
    fn from_bits(bits: &[u8]) -> Self {
        bit_to_usize_helper(bits) as u8
    }
}
impl ValueFromBits for i8 {
    fn from_bits(bits: &[u8]) -> Self {
        bit_to_usize_helper(bits) as i8
    }
}

impl ValueFromBits for f32 {
    fn from_bits(bits: &[u8]) -> Self {
        bit_to_usize_helper(bits) as f32
    }
}

impl ValueFromBits for String {
    fn from_bits(bits: &[u8]) -> Self {
        bits.chunks(6)
            .map(|chunk| STRING_CONVERSION_LOOKUP[bit_to_usize_helper(chunk)])
            .filter(|&c| c != "@")
            .collect::<String>()
            .trim()
            .to_string()
    }
}
