//use super::ast::aexpr::AExpr;
//use super::ast::aexpr::AExp;
//use super::ast::aexpr;
use super::ast::AExpr;
use crate::while_lang::State;
use crate::while_lang::Value;

pub fn eval (ast: Box<AExpr>, state: &State) -> Result<Value, String> {
    return match *ast {
        AExpr::Value(v) => Ok(v),
        AExpr::Variable(v) => state.get(&v),
        AExpr::Add(a, b) => evalBinary(|a, b| a + b, a, b, state),
        AExpr::Sub(a, b) => evalBinary(|a, b| a - b, a, b, state),
        AExpr::Mul(a, b) => evalBinary(|a, b| a * b, a, b, state),
    }
}
fn evalBinary <F>(f: F, left: Box<AExpr>, right: Box<AExpr>, state: &State) -> Result<Value, String>
    where F: FnOnce(Value, Value) -> Value
{
    return match eval(left, state) {
        Ok(a) => match eval(right, state) {
            Ok(b) => Ok(f(a, b)),
            err => err,
        },
        err => err
    };
}
