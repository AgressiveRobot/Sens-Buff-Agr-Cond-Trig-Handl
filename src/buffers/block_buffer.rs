use crate::datatypes::Pack;

pub struct BlockBuffer {
    sum: i64,
    min_val: i64,
    max_val: i64,
    count: usize,
}

impl BlockBuffer {
    pub fn new() -> Self {
        Self { sum: 0, min_val: i64::MAX, max_val: i64::MIN, count: 0 }
    }

    // Принимает Pack<N>, вытаскивает значение по индексу val_index, 
    // копит блок размером block_size и возвращает сжатый Pack<3> (avg, min, max)
    pub fn push_block<const N: usize>(&mut self, pack: &Pack<N>, val_index: usize, block_size: usize) -> Option<Pack<3>> {
        if let Some(val) = pack.values.get(val_index) {
            let val = *val;
            self.sum += val;
            self.count += 1;

            if val < self.min_val { self.min_val = val; }
            if val > self.max_val { self.max_val = val; }

            if self.count == block_size {
                let avg = self.sum / self.count as i64;
                let out_pack = Pack::<3> {
                    values: [avg, self.min_val, self.max_val],
                    count: 3,
                };
                // Полный сброс эпохи
                self.sum = 0; self.count = 0;
                self.min_val = i64::MAX; self.max_val = i64::MIN;
                
                return Some(out_pack);
            }
        }
        None
    }
}
