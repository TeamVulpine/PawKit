#![allow(incomplete_features)]
#![feature(decl_macro, variant_count, generic_const_exprs)]

use std::ops::Deref;

use pawkit_crockford::Ulid;
use pawkit_interner::InternString;
use serde::{Deserialize, Serialize};

pub mod binding;
pub mod manager;
pub mod state;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BindingName(InternString);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId(Ulid);

impl BindingName {
    pub fn new(name: &str) -> Self {
        return Self(InternString::new(name));
    }
}

impl Deref for BindingName {
    type Target = InternString;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl DeviceId {
    pub fn null() -> Self {
        return Self(Ulid::from_raw_parts(0, 0));
    }
}

fn length_squared([a, b]: [f32; 2]) -> f32 {
    return a * a + b * b;
}
