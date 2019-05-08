
#[derive(Debug)]
pub enum AExpr {
    Value(i64),
    Add(Box<AExpr>, Box<AExpr>),
    Sub(Box<AExpr>, Box<AExpr>),
}
pub type AExp = Box<AExpr>;
pub fn value (v: i64) -> AExp {
    return Box::new(AExpr::Value(v));
}

use std::ops;
impl ops::Add<AExp> for AExp {
    type Output = AExp;
    fn add (self, rhs: AExp) -> AExp {
        return Box::new(AExpr::Add(self, rhs));
    }
}
impl ops::Sub<AExp> for AExp {
    type Output = AExp;
    fn sub (self, rhs: AExp) -> AExp {
        return Box::new(AExpr::Sub(self, rhs));
    }
}
fn foo () -> AExp {
    return value(10) + value(20);
}
