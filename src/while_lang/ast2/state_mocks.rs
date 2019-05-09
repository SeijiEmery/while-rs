use super::state::{ State, Value, VResult };

#[derive(Debug, PartialEq)]
pub struct MockEmptyState();
impl MockEmptyState { pub fn new () -> MockEmptyState { MockEmptyState() } }
impl State for MockEmptyState {
    fn get (&self, var: &str) -> VResult { Err(format!("undefined variable '{}'!", var)) }
    fn set (&mut self, _: &str, _: Value) {}
}

#[derive(Debug, PartialEq)]
pub struct MockStateWithVar { var: String, val: Value }
impl MockStateWithVar {
    pub fn new (var: &str, val: Value) -> MockStateWithVar {
        MockStateWithVar { var: var.to_string(), val }
    }
}
impl State for MockStateWithVar {
    fn get (&self, var: &str) -> VResult {
        if var == self.var {
            Ok(self.val)
        } else {
            Err(format!("undefined variable '{}'!", var))
        }
    }
    fn set (&mut self, var: &str, val: Value) {}
}