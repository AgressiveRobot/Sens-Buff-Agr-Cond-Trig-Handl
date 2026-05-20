pub struct StackRingBuffer<T, const CAPACITY: usize> {
    data: [Option<T>; CAPACITY],
    head: usize,
    tail: usize,
    pub count: usize,
}

impl<T: Copy, const CAPACITY: usize> StackRingBuffer<T, CAPACITY> {
    pub fn new() -> Self {
        Self { data: [None; CAPACITY], head: 0, tail: 0, count: 0 }
    }

    pub fn push(&mut self, item: T) {
        if self.count == CAPACITY {
            self.head = (self.head + 1) % CAPACITY;
        } else {
            self.count += 1;
        }
        self.data[self.tail] = Some(item);
        self.tail = (self.tail + 1) % CAPACITY;
    }

    pub fn get_latest(&self) -> Option<T> {
        if self.count == 0 { return None; }
        let idx = if self.tail == 0 { CAPACITY - 1 } else { self.tail - 1 };
        self.data[idx]
    }

    // Итератор по активным элементам буфера (нужен для agr_funcs)
    pub fn iter(&self) -> BufferIter<'_, T, CAPACITY> {
        BufferIter { buffer: self, current: 0 }
    }
}

pub struct BufferIter<'a, T, const CAP: usize> {
    buffer: &'a StackRingBuffer<T, CAP>,
    current: usize,
}

impl<'a, T: Copy, const CAP: usize> Iterator for BufferIter<'a, T, CAP> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.buffer.count {
            let idx = (self.buffer.head + self.current) % CAP;
            self.current += 1;
            self.buffer.data[idx]
        } else { None }
    }
}
