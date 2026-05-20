// src/types.rs

// 1. Абсолютно чистая структура. Никакого упоминания времени или ID.
#[derive(Debug, Clone, Copy)]
pub struct Pack<const N: usize> {
    pub values: [i32; N], // Просто плоская лента из i32 на стеке
    pub count: usize,
}

// 2. Базовый интерфейс для чтения
pub trait DataChunk {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Option<i32>;
}

impl<const N: usize> DataChunk for Pack<N> {
    fn size(&self) -> usize { self.count }
    fn get(&self, index: usize) -> Option<i32> {
        if index < self.count { Some(self.values[index]) } else { None }
    }
}

// 3. Отдельный независимый модуль склейки! 
// Он работает с любым Pack, которому мы укажем нужные индексы ячеек
pub trait BitPacker {
    fn write_u64(&mut self, hi_idx: usize, lo_idx: usize, val: u64);
    fn read_u64(&self, hi_idx: usize, lo_idx: usize) -> u64;
    
    fn write_i64(&mut self, hi_idx: usize, lo_idx: usize, val: i64);
    fn read_i64(&self, hi_idx: usize, lo_idx: usize) -> i64;
}

// Реализуем склейку для Pack любой длины
impl<const N: usize> BitPacker for Pack<N> {
    // Записать u64 в две ячейки i32
    fn write_u64(&mut self, hi_idx: usize, lo_idx: usize, val: u64) {
        self.values[hi_idx] = (val >> 32) as u32 as i32;
        self.values[lo_idx] = (val & 0xFFFFFFFF) as u32 as i32;
    }

    // Прочитать u64 из двух ячеек i32
    fn read_u64(&self, hi_idx: usize, lo_idx: usize) -> u64 {
        let hi = self.values[hi_idx] as u32;
        let lo = self.values[lo_idx] as u32;
        ((hi as u64) << 32) | (lo as u64)
    }

    // Записать знаковое i64 (например, огромный счетчик со знаком)
    fn write_i64(&mut self, hi_idx: usize, lo_idx: usize, val: i64) {
        // Для процессора i64 и u64 на битовом уровне идентичны, переводим в u64 для безопасного сдвига
        self.write_u64(hi_idx, lo_idx, val as u64);
    }

    // Прочитать знаковое i64
    fn read_i64(&self, hi_idx: usize, lo_idx: usize) -> i64 {
        self.read_u64(hi_idx, lo_idx) as i64
    }
}

// Плоские интерфейсы автоматизации
pub trait Condition { fn check(&self, value: i32) -> bool; }
pub trait Handler { fn execute(&self, msg: &str, trigger_value: i32); }
pub trait Trigger { fn check(&self, msg: &str, value: i32); }
