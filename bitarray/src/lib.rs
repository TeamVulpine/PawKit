pub struct BitArray {
    data: Box<[u8]>,
}

impl BitArray {
    pub fn new(len: usize) -> Self {
        return Self {
            data: vec![0; (len + 7) >> 3].into_boxed_slice(),
        };
    }

    pub fn len(&self) -> usize {
        return self.data.len() * 8;
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len() {
            return None;
        }

        // SAFETY: We just bounds checked.
        // The Rust compiler doesn't understand enough about our code to know that the size of the data is self.len() >> 3.
        unsafe {
            let value = self.data.get_unchecked(index >> 3);

            return Some((*value & (1 << (index & 7))) != 0);
        }
    }

    pub fn set(&mut self, index: usize) {
        if index >= self.len() {
            return;
        }

        // SAFETY: We just bounds checked.
        // The Rust compiler doesn't understand enough about our code to know that the size of the data is self.len() >> 3.
        unsafe {
            let value = self.data.get_unchecked_mut(index >> 3);

            *value |= 1 << (index & 7);
        }
    }

    pub fn reset(&mut self, index: usize) {
        if index >= self.len() {
            return;
        }

        // SAFETY: We just bounds checked.
        // The Rust compiler doesn't understand enough about our code to know that the size of the data is self.len() >> 3.
        unsafe {
            let value = self.data.get_unchecked_mut(index >> 3);

            *value &= !(1 << (index & 7));
        }
    }

    pub fn clear(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = 0;
        }
    }

    pub fn fill(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = 0xFF;
        }
    }
}
