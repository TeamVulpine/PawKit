use std::{ops::Deref, sync::OnceLock};

use crate::InternString;

pub struct LazyInternString<'a> {
    value: &'a str,
    lock: OnceLock<InternString>,
}

impl<'a> LazyInternString<'a> {
    pub const fn new(s: &'a str) -> Self {
        return Self {
            value: s,
            lock: OnceLock::new(),
        };
    }

    pub fn get(&self) -> &InternString {
        return self.lock.get_or_init(|| InternString::new(self.value));
    }
}

impl<'a> Deref for LazyInternString<'a> {
    type Target = InternString;

    fn deref(&self) -> &Self::Target {
        return self.get();
    }
}
