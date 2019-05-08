//use super::ast::aexpr::AExpr;
//use super::ast::aexpr::AExp;
//use super::ast::aexpr;
use super::ast::AExpr;

pub fn eval (ast: Box<AExpr>) -> i64 {
    return match *ast {
        AExpr::Value(v) => v,
        AExpr::Variable(v) => 0,
        AExpr::Add(a, b) => eval(a) + eval(b),
        AExpr::Sub(a, b) => eval(a) - eval(b),
        AExpr::Mul(a, b) => eval(a) * eval(b),
    }
}
