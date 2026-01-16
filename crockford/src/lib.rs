#![feature(decl_macro)]

use std::{
    fmt::Debug,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::RngCore;

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

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ulid([u8; 16]);

impl Ulid {
    pub fn from_raw_parts(timestamp: u128, random: u128) -> Self {
        let timestamp = timestamp.to_be_bytes();
        let random = random.to_be_bytes();

        let mut bytes = [0u8; 16];

        bytes[..6].copy_from_slice(&timestamp[10..]);
        bytes[6..].copy_from_slice(&random[6..]);

        return Self(bytes);
    }

    pub fn into_raw_parts(&self) -> (u128, u128) {
        let mut timestamp_bytes = [0u8; 16];
        timestamp_bytes[10..].copy_from_slice(self.timestamp_bytes());

        let mut random_bytes = [0u8; 16];
        random_bytes[6..].copy_from_slice(self.random_bytes());

        let timestamp = u128::from_be_bytes(timestamp_bytes);
        let random = u128::from_be_bytes(random_bytes);

        return (timestamp, random);
    }

    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let timestamp = timestamp.to_be_bytes();

        let mut bytes = [0u8; 16];

        bytes[..6].copy_from_slice(&timestamp[10..]);

        rand::rng().fill_bytes(&mut bytes[6..]);

        return Self(bytes);
    }

    pub fn timestamp_bytes(&self) -> &[u8] {
        return &self.0[..6];
    }

    pub fn random_bytes(&self) -> &[u8] {
        return &self.0[6..];
    }
}

impl Debug for Ulid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f
            .debug_tuple("Ulid")
            .field(&self.into_crockford(26))
            .finish();
    }
}

impl From<Ulid> for u128 {
    fn from(value: Ulid) -> Self {
        return Self::from_be_bytes(value.0);
    }
}

impl From<u128> for Ulid {
    fn from(value: u128) -> Self {
        return Self(value.to_be_bytes());
    }
}

impl From<Ulid> for [u8; 16] {
    fn from(value: Ulid) -> [u8; 16] {
        return value.0;
    }
}

impl From<[u8; 16]> for Ulid {
    fn from(bytes: [u8; 16]) -> Ulid {
        return Ulid(bytes);
    }
}

impl IntoCrockford for Ulid {
    fn into_crockford(&self, _: usize) -> String {
        let (timestamp, random) = self.into_raw_parts();

        return timestamp.into_crockford(10) + &random.into_crockford(16);
    }
}

impl FromCrockford for Ulid {
    fn from_crockford(value: &str) -> Option<Self>
    where
        Self: Sized,
    {
        let (timestamp, random) = value.split_at_checked(10)?;

        let timestamp = u128::from_crockford(timestamp)?;
        let random = u128::from_crockford(random)?;

        return Some(Self::from_raw_parts(timestamp, random));
    }
}
