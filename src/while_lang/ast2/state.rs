
pub type Value = i64;
pub type Variable = String;
pub type VResult = Result<Value, String>;
pub trait State {
    fn get (&self, var: &str) -> VResult;
    fn set (&mut self, var: &str, val: Value);
}
