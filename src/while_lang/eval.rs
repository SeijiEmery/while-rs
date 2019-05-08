//use super::ast::aexpr::AExpr;
//use super::ast::aexpr::AExp;
//use super::ast::aexpr;
use super::ast::AExpr;
use crate::while_lang::State;
use crate::while_lang::Value;

pub fn eval (ast: Box<AExpr>, state: &State) -> Value {
    return match *ast {
        AExpr::Value(v) => Value::Int(v),
        AExpr::Variable(v) => state.get(&v),
        AExpr::Add(a, b) => eval(a, state) + eval(b, state),
        AExpr::Sub(a, b) => eval(a, state) - eval(b, state),
        AExpr::Mul(a, b) => eval(a, state) * eval(b, state),
    }
}
