use crate::datatypes::Condition;

pub struct LessThan { pub threshold: i64 }
impl Condition for LessThan {
    fn check(&self, value: i64) -> bool { value < self.threshold }
}

pub struct MoreThan { pub threshold: i64 }
impl Condition for MoreThan {
    fn check(&self, value: i64) -> bool { value > self.threshold }
}
