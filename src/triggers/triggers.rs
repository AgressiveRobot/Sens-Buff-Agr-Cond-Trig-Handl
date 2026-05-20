use crate::datatypes::{Trigger, Condition, Handler};

pub struct GenericTrigger<C: Condition, H: Handler> {
    condition: C,
    handler: H,
}

impl<C: Condition, H: Handler> GenericTrigger<C, H> {
    pub fn new(condition: C, handler: H) -> Self {
        Self { condition, handler }
    }
}

impl<C: Condition, H: Handler> Trigger for GenericTrigger<C, H> {
    fn check(&self, msg: &str, value: i64) {
        // Если условие выполнилось — дергаем хэндлер
        if self.condition.check(value) {
            self.handler.execute(msg, value);
        }
    }
}
