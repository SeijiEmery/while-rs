use std::collections::HashMap;

pub type Value = i64;
pub type Variable = String;
pub type VResult = Result<Value, String>;
pub trait State {
    fn get (&self, var: &str) -> VResult;
    fn set (&mut self, var: &str, val: Value);
}

// Wrap HashMap but only provide our State's public interface
pub struct HashState(HashMap<Variable, Value>);
impl HashState {
    pub fn new () -> HashState { HashState(HashMap::new()) }


    pub fn is_empty (&self) -> bool { self.0.is_empty() }
}
impl State for HashState {
    fn get (&self, v: &str) -> VResult {
        match self.0.get(v) {
            Some(a) => Ok(*a),
            None => Err(format!("Undefined variable '{}'!", v))
        }
    }
    fn set (&mut self, v: &str, a: Value) {
        self.0.insert(v.to_string(), a);
    }
}
