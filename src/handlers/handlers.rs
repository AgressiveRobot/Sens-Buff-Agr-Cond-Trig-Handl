use crate::datatypes::Handler;

pub struct ConsolePrintlnHandler;
impl ConsolePrintlnHandler { pub fn new() -> Self { Self } }

impl Handler for ConsolePrintlnHandler {
    fn execute(&self, msg: &str, trigger_value: i64) {
        // Делим на 100 для красивого вывода фиксированной запятой
        println!("  [CONSOLE ALERT] -> {} Значение: {:.2}", msg, trigger_value as f64 / 100.0);
    }
}
