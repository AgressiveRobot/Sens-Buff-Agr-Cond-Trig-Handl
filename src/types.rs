// Единственная структура данных в системе на Const Generics (лежит строго на стеке)
#[derive(Debug, Clone, Copy)]
pub struct Pack<const N: usize> {
    pub values: [i64; N], // По индексам лежит всё: ID, время, метрики
    pub count: usize,     // Сколько реально ячеек занято
}

// Интерфейс для чтения данных из любого Пака
pub trait DataChunk {
    fn size(&self) -> usize;
    fn get(&self, index: usize) -> Option<i64>;
}

impl<const N: usize> DataChunk for Pack<N> {
    fn size(&self) -> usize { self.count }
    fn get(&self, index: usize) -> Option<i64> {
        if index < self.count { Some(self.values[index]) } else { None }
    }
}

// Интерфейс для плоских условий (больше, меньше, равно)
pub trait Condition {
    fn check(&self, value: i64) -> bool;
}

// Интерфейс для исполнительных ручек (Консоль, Телеграм, База данных)
pub trait Handler {
    fn execute(&self, msg: &str, trigger_value: i64);
}

// Интерфейс триггера, который связывает Condition и Handler
pub trait Trigger {
    fn check(&self, msg: &str, value: i64);
}

// Допиши это в конец существующего файла src/datatypes.rs

pub trait Sender<const N: usize> {
    // Отправляет Pack<N> по выбранному каналу
    fn send(&mut self, pack: &Pack<N>) -> Result<(), &'static str>;
}

pub trait Receiver<const N: usize> {
    // Слушает сокет/порт и собирает данные прямо в Pack<N> на стеке
    // Возвращает Some(Pack<N>) когда данные пришли, или None если таймаут/ошибка
    fn receive(&mut self) -> Option<Pack<N>>;
}

