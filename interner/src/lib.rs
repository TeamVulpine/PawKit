#![feature(cold_path, str_from_raw_parts)]

use core::{fmt, str};
use std::{
    alloc::{Layout, alloc, dealloc},
    fmt::{Debug, Display},
    hint::cold_path,
    mem::forget,
    ops::Deref,
    ptr::{self, NonNull, copy_nonoverlapping},
    str::FromStr,
    sync::{
        LazyLock,
        atomic::{AtomicUsize, Ordering},
    },
};

use dashmap::{DashMap, Entry};
use serde::{Deserialize, Serialize, de::Visitor};

const MAX_REFCOUNT: usize = (isize::MAX) as usize;

struct InternInner {
    strong: AtomicUsize,
    weak: AtomicUsize,
    len: usize,
}

/// Represents an interned string.
/// Similar to an Arc<str>, but it also deduplicates strings.
/// It is designed for fast forward lookup and cloning, with reverse lookup being prioritized over insertions / deletions.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternString {
    inner: NonNull<InternInner>,
}

/// A weak interned string.
/// Similar to Weak<str>, but references an InternString instead
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WeakInternString {
    inner: NonNull<InternInner>,
}

/// # SAFETY
/// The keys are casted to a static lifetime, and reference the underlying InternInner data,
/// but the keys are removed before the InternInner is freed.
static DATA: LazyLock<DashMap<&'static str, WeakInternString>> = LazyLock::new(Default::default);

impl InternInner {
    unsafe fn data_ptr<'a>(value: NonNull<Self>) -> *const u8 {
        unsafe {
            return value.as_ptr().add(1) as *const u8;
        }
    }

    unsafe fn data_ptr_mut<'a>(value: NonNull<Self>) -> *mut u8 {
        unsafe {
            return value.as_ptr().add(1) as *mut u8;
        }
    }

    unsafe fn data_mut<'a>(value: NonNull<Self>) -> &'a mut str {
        unsafe {
            return str::from_raw_parts_mut(Self::data_ptr_mut(value), (*value.as_ptr()).len);
        }
    }

    unsafe fn data<'a>(value: NonNull<Self>) -> &'a str {
        unsafe {
            return str::from_raw_parts(Self::data_ptr(value), (*value.as_ptr()).len);
        }
    }

    fn layout_for(len: usize) -> Layout {
        return Layout::new::<Self>()
            .extend(Layout::array::<u8>(len).unwrap())
            .unwrap()
            .0
            .pad_to_align();
    }

    fn layout(value: NonNull<Self>) -> Layout {
        unsafe {
            return Self::layout_for((*value.as_ptr()).len);
        }
    }

    unsafe fn alloc(s: &str) -> (NonNull<Self>, &'static str) {
        unsafe {
            let layout = Self::layout_for(s.len());

            let ptr = alloc(layout) as *mut Self;

            let value = &mut *ptr;

            value.strong = AtomicUsize::new(1);
            value.weak = AtomicUsize::new(1);
            value.len = s.len();

            let ptr = NonNull::new_unchecked(ptr);
            let data = InternInner::data_mut(ptr);

            copy_nonoverlapping(s.as_ptr(), data.as_mut_ptr(), s.len());

            return (ptr, data);
        }
    }

    unsafe fn dealloc(value: NonNull<Self>) {
        unsafe {
            let ptr = value.as_ptr();
            let layout = Self::layout(value);

            dealloc(ptr as *mut u8, layout);
        }
    }
}

impl InternString {
    fn inner(&self) -> &InternInner {
        unsafe {
            return &*self.inner.as_ptr();
        }
    }

    /// Consumes the `InternString`, returning the wrapped pointer.
    ///
    /// To avoid a memory leak the pointer must be converted back to an `InternString` using
    /// [`InternString::from_raw`].
    pub fn into_raw(self) -> *const u8 {
        let ptr = self.inner.as_ptr() as *const u8;

        forget(self);

        return ptr;
    }

    /// Constructs an `InternString` from a raw pointer.
    ///
    /// The raw pointer must have been previously returned by a call to [`InternString::into_raw`].
    pub unsafe fn from_raw(value: *const u8) -> Option<Self> {
        let inner = NonNull::new(value as *mut InternInner)?;
        if !inner.is_aligned() {
            return None;
        }

        return Some(Self { inner: inner });
    }

    /// Creates a new intern string from the provided string
    /// If an intern string exists with the same content, it returns a clone of that.
    pub fn new(s: &str) -> Self {
        if let Some(weak) = DATA.get(s) {
            let value = Self { inner: weak.inner };

            value.inner().strong.fetch_add(1, Ordering::Relaxed);

            return value;
        }

        let (ptr, str) = unsafe { InternInner::alloc(s) };

        match DATA.entry(str) {
            Entry::Occupied(occupied) => {
                cold_path();

                unsafe {
                    InternInner::dealloc(ptr);
                }

                let value = Self {
                    inner: occupied.get().inner,
                };

                value.inner().strong.fetch_add(1, Ordering::Relaxed);

                return value;
            }
            Entry::Vacant(vacant) => {
                vacant.insert(WeakInternString { inner: ptr });

                let value = Self { inner: ptr };

                return value;
            }
        }
    }

    /// Retruns the underlying string data.
    pub fn as_str<'a>(&'a self) -> &'a str {
        unsafe {
            return InternInner::data(self.inner);
        }
    }

    /// Converts into a weak reference
    pub fn into_weak(&self) -> WeakInternString {
        let weak = WeakInternString { inner: self.inner };

        self.inner().weak.fetch_add(1, Ordering::Relaxed);

        return weak;
    }
}

impl WeakInternString {
    fn inner(&self) -> &InternInner {
        unsafe {
            return &*self.inner.as_ptr();
        }
    }

    /// Returns true if the string is still alive
    /// The string is considered dead if there are no strong references
    pub fn is_alive(&self) -> bool {
        return self.inner().strong.load(Ordering::Acquire) != 0;
    }

    /// Gets the underlying string if it's still alive
    pub fn as_str<'a>(&'a self) -> Option<&'a str> {
        if !self.is_alive() {
            return None;
        }

        unsafe {
            return Some(InternInner::data(self.inner));
        }
    }

    /// Converts to a strong reference if the string is still alive
    pub fn into_strong(&self) -> Option<InternString> {
        #[inline]
        fn checked_increment(n: usize) -> Option<usize> {
            if n == 0 {
                return None;
            }
            assert!(n <= MAX_REFCOUNT);
            return Some(n + 1);
        }

        self.inner()
            .strong
            .fetch_update(Ordering::Acquire, Ordering::Relaxed, checked_increment)
            .ok()?;

        return Some(InternString { inner: self.inner });
    }
}

unsafe impl Send for InternString {}
unsafe impl Sync for InternString {}

unsafe impl Send for WeakInternString {}
unsafe impl Sync for WeakInternString {}

impl Drop for InternString {
    fn drop(&mut self) {
        if self.inner().strong.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }

        self.inner().strong.load(Ordering::Acquire);

        DATA.remove(self.as_str());
    }
}

impl Drop for WeakInternString {
    fn drop(&mut self) {
        if self.inner().weak.fetch_sub(1, Ordering::Release) == 1 {
            self.inner().weak.load(Ordering::Acquire);

            unsafe {
                InternInner::dealloc(self.inner);
            }
        }
    }
}

impl Clone for InternString {
    fn clone(&self) -> Self {
        let ref_count = self.inner().strong.fetch_add(1, Ordering::Relaxed);

        assert!(ref_count < MAX_REFCOUNT);

        return Self {
            inner: self.inner.clone(),
        };
    }
}

impl Clone for WeakInternString {
    fn clone(&self) -> Self {
        let ref_count = self.inner().weak.fetch_add(1, Ordering::Relaxed);

        assert!(ref_count < MAX_REFCOUNT);

        return Self {
            inner: self.inner.clone(),
        };
    }
}

impl PartialEq<WeakInternString> for InternString {
    fn eq(&self, other: &WeakInternString) -> bool {
        return ptr::addr_eq(self.inner.as_ptr(), other.inner.as_ptr());
    }
}

impl PartialEq<InternString> for WeakInternString {
    fn eq(&self, other: &InternString) -> bool {
        return ptr::addr_eq(self.inner.as_ptr(), other.inner.as_ptr());
    }
}

impl PartialEq<&str> for InternString {
    fn eq(&self, other: &&str) -> bool {
        return self.as_str() == *other;
    }
}

impl PartialEq<&str> for WeakInternString {
    fn eq(&self, other: &&str) -> bool {
        let Some(str) = self.as_str() else {
            return false;
        };

        return str == *other;
    }
}

impl Deref for InternString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        return self.as_str();
    }
}

impl Debug for InternString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.debug_tuple("InternString").field(&self.as_str()).finish();
    }
}

impl Display for InternString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return f.write_str(self.as_str());
    }
}

impl Debug for WeakInternString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut t = f.debug_tuple("WeakInternString");

        let Some(s) = self.as_str() else {
            return t.field(&"<dead>").finish();
        };

        return t.field(&s).finish();
    }
}

impl Display for WeakInternString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Some(s) = self.as_str() else {
            return f.write_str(&"<dead>");
        };

        return f.write_str(&s);
    }
}

impl FromStr for InternString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(Self::new(s));
    }
}

impl From<&str> for InternString {
    fn from(value: &str) -> Self {
        return Self::new(value);
    }
}

impl From<String> for InternString {
    fn from(value: String) -> Self {
        return Self::new(&value);
    }
}

impl Serialize for InternString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.serialize_str(self);
    }
}

struct InternStringVisitor;

impl<'a> Visitor<'a> for InternStringVisitor {
    type Value = InternString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.into())
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match str::from_utf8(v) {
            Ok(s) => Ok(s.into()),
            Err(_) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bytes(v),
                &self,
            )),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match String::from_utf8(v) {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Bytes(&e.into_bytes()),
                &self,
            )),
        }
    }
}

impl<'de> Deserialize<'de> for InternString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        return deserializer.deserialize_string(InternStringVisitor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    #[test]
    fn basic_interning() {
        let s1 = InternString::new("hello");
        let s2 = InternString::new("hello");
        let s3 = InternString::new("world");

        assert_eq!(s1, s2);
        assert_ne!(s1, s3);

        assert_eq!(s1.as_str(), "hello");
        assert_eq!(s3.as_str(), "world");
    }

    #[test]
    fn weak_references() {
        let s1 = InternString::new("weak_test");
        let w1 = s1.into_weak();

        assert!(w1.is_alive());
        assert_eq!(w1.as_str(), Some("weak_test"));

        drop(s1);

        assert!(!w1.is_alive());
        assert_eq!(w1.as_str(), None);
    }

    #[test]
    fn cloning_strong() {
        let s1 = InternString::new("clone_test");
        let s2 = s1.clone();
        let s3 = s2.clone();

        assert_eq!(s1.inner.as_ptr(), s2.inner.as_ptr());
        assert_eq!(s2.inner.as_ptr(), s3.inner.as_ptr());

        assert_eq!(s1.as_str(), "clone_test");
    }

    #[test]
    fn cloning_weak() {
        let s1 = InternString::new("weak_clone");
        let w1 = s1.into_weak();
        let w2 = w1.clone();

        assert_eq!(w1.inner.as_ptr(), w2.inner.as_ptr());

        drop(s1);

        assert!(!w1.is_alive());
        assert!(!w2.is_alive());
    }

    #[test]
    fn eq_partial() {
        let s = InternString::new("eq_test");
        let w = s.into_weak();

        assert_eq!(s, w);
        assert_eq!(w, s);

        assert_eq!(s, "eq_test");
        assert_eq!(w, "eq_test");
    }

    #[test]
    fn data_deduplication() {
        let s1 = InternString::new("dedup");
        let s2 = InternString::new("dedup");

        assert!(ptr::addr_eq(s1.inner.as_ptr(), s2.inner.as_ptr()));
    }

    #[test]
    fn drop_cleans_data() {
        let s = InternString::new("cleanup_test");

        drop(s);

        assert!(DATA.get("cleanup_test").is_none());
    }

    #[test]
    fn multithreaded_usage() {
        let s = Arc::new(InternString::new("thread_test"));

        let mut handles = vec![];

        for _ in 0..10 {
            let s_clone = Arc::clone(&s);
            handles.push(thread::spawn(move || {
                let local = s_clone.clone();
                assert_eq!(local.as_str(), "thread_test");
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(s.as_str(), "thread_test");
    }

    #[test]
    fn weak_after_drop_multithreaded() {
        let s = InternString::new("weak_thread");
        let w = s.into_weak();

        let handle = thread::spawn(move || {
            drop(s);
        });

        handle.join().unwrap();

        assert!(!w.is_alive());
        assert_eq!(w.as_str(), None);
    }

    #[test]
    fn simultaneous_intern() {
        let barrier = Arc::new(Barrier::new(2));
        let s1 = Arc::new(Mutex::new(None));
        let s2 = Arc::new(Mutex::new(None));

        let b1 = barrier.clone();
        let s1c = s1.clone();
        let t1 = thread::spawn(move || {
            b1.wait();
            *s1c.lock().unwrap() = Some(InternString::new("race_test"));
        });

        let b2 = barrier.clone();
        let s2c = s2.clone();
        let t2 = thread::spawn(move || {
            b2.wait();
            *s2c.lock().unwrap() = Some(InternString::new("race_test"));
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let s1 = s1.lock().unwrap().take().unwrap();
        let s2 = s2.lock().unwrap().take().unwrap();

        assert!(ptr::addr_eq(s1.inner.as_ptr(), s2.inner.as_ptr()));
    }

    #[test]
    fn weak_upgrade() {
        let s = InternString::new("upgrade_test");
        let w = s.into_weak();

        let s2 = w.into_strong().expect("Should upgrade");
        assert_eq!(s2.as_str(), "upgrade_test");

        drop(s);
        drop(s2);

        assert!(w.into_strong().is_none());
    }
}
