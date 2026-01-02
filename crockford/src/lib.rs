#![feature(decl_macro)]

pub const CROCKFORD_DIGITS: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

pub fn crockford_char_to_digit(c: char) -> Option<u8> {
    return Some(match c.to_ascii_uppercase() {
        '0' | 'O' => 0,
        '1' | 'I' | 'L' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        'G' => 16,
        'H' => 17,
        'J' => 18,
        'K' => 19,
        'M' => 20,
        'N' => 21,
        'P' => 22,
        'Q' => 23,
        'R' => 24,
        'S' => 25,
        'T' => 26,
        'V' => 27,
        'W' => 28,
        'X' => 29,
        'Y' => 30,
        'Z' => 31,
        _ => return None,
    });
}

pub trait IntoCrockford {
    fn into_crockford(&self, padding: usize) -> String;
}

pub trait FromCrockford {
    fn from_crockford(value: &str) -> Option<Self>
    where
        Self: Sized;
}

macro impl_crockford($t:ty) {
    impl IntoCrockford for $t {
        fn into_crockford(&self, padding: usize) -> String {
            if *self == 0 {
                return "0".repeat(padding.max(1));
            }

            let mut value = *self;
            let mut result = Vec::new();

            while value > 0 {
                let rem = (value % 32) as usize;
                result.push(CROCKFORD_DIGITS[rem] as char);
                value >>= 5;
            }

            let mut encoded: String = result.iter().rev().collect();
            if encoded.len() < padding {
                let pad_len = padding - encoded.len();
                encoded = "0".repeat(pad_len) + &encoded;
            }

            encoded
        }
    }

    impl FromCrockford for $t {
        fn from_crockford(value: &str) -> Option<Self> {
            let mut result: $t = 0;

            for c in value.chars() {
                let digit = crockford_char_to_digit(c)?;
                result = result.checked_mul(32)?.checked_add(digit as $t)?;
            }

            Some(result)
        }
    }
}

impl_crockford!(u8);
impl_crockford!(u16);
impl_crockford!(u32);
impl_crockford!(u64);
impl_crockford!(u128);
