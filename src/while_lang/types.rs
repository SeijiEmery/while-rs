use std::collections::HashMap;

pub type Value = i64;
pub type Variable = String;
pub type Evaluation = Result<Value, String>;

pub trait State {
    fn get (&self, var: &str) -> Result<Value, String>;
    fn set (&mut self, var: &str, val: Value);
}

pub type HashState = HashMap<Variable, Value>;
impl State for HashState {
    fn get(&self, var: &str) -> Evaluation {
        return match self.get(var) {
            Some(value) => Ok(*value),
            None => Err(format!("Undefined variable '{}'", var)),
        };
    }
    fn set(&mut self, var: &str, val: Value) {
        self.insert(var.to_string(), val);
    }
}
