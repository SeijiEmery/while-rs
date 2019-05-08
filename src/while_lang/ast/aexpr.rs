
#[derive(Debug)]
pub enum AExpr {
    Value(i64),
    Add(AExpr, AExpr),
    Sub(AExpr, AExpr),
}
pub fn eval (ast: AExpr) -> i64 {
    return match ast {
        AExpr::Value(v) => v,
        AExpr::Add(a, b) => eval(a) + eval(b),
        AExpr::Sub(a, b) => eval(a) - eval(b),
    }
}
