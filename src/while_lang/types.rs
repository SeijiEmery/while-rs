use std::collections::HashMap;

pub type IntegerValue = i64;
pub type Variable = String;

#[derive(Debug, Clone)]
pub enum Value {
    Int(IntegerValue),
    None
}
pub trait State {
    fn get (&self, var: &str) -> Value;
    fn set (&mut self, var: &str, val: Value);
}

pub type HashState = HashMap<Variable, Value>;
impl State for HashState {
    fn get (&self, var: &str) -> Value {
        return match self.get(var) {
            Some(value) => *value,
            None => Value::None,
        };
    }
    fn set (&mut self, var: &str, val: Value) {
        use Value;
        match val {
            Value::Int(x) => self.insert(String::from(var), val),
            Value::None => self.remove(var),
        };
impl PartialEq for Value {
    fn eq (&self, other: &Value) -> bool {
        return match self {
            Value::None => match other {
                Value::None => true,
                _ => false
            },
            Value::Int(a) => match other {
                Value::Int(b) => a == b,
                _ => false
            }
        }
    }
}
impl
