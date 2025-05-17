/// An array that allows holes in data, and automatically manages free slots
#[derive(Debug)]
pub struct HolyArray<T> {
    data: Vec<Option<T>>,
    free_slots: Vec<usize>,
}

unsafe impl<T> Send for HolyArray<T> where T: Send {}

impl<T> HolyArray<T> {
    pub fn new() -> Self {
        return Self {
            data: vec![],
            free_slots: vec![],
        };
    }

    pub fn acquire(&mut self, value: T) -> usize {
        let Some(idx) = self.free_slots.pop() else {
            let idx = self.data.len();
            self.data.push(Some(value));
            return idx;
        };

        self.data[idx] = Some(value);

        return idx;
    }

    pub fn release(&mut self, index: usize) {
        if index >= self.data.len() {
            return;
        }

        if self.data[index].is_none() {
            return;
        }

        self.data[index] = None;
        self.free_slots.push(index);
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let Some(Some(value)) = self.data.get(index) else {
            return None;
        };

        return Some(value);
    }
}
