//use super::ast::aexpr::AExpr;
//use super::ast::aexpr::AExp;
//use super::ast::aexpr;
use super::ast::AExpr;

pub fn eval <'a> (ast: &'a AExpr) -> i64 {
    return match ast {
        AExpr::Value(v) => *v,
        AExpr::Add(a, b) => eval(a) + eval(b),
        AExpr::Sub(a, b) => eval(a) - eval(b),
    }
}
