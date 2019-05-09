use super::state::State;
use std::fmt;

pub trait Expr <Value, AST>: fmt::Debug {
    fn eval (&self, state: &State) -> Result<Value, String>;
    fn eval1 (&self, state: &State) -> Result<AST, String>;
    fn is_reduced (&self) -> bool;
}
