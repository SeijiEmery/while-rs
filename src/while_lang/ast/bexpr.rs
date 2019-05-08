use crate::while_lang::types::Value;
use aexpr::AExpr;

#[derive(Debug)]
pub enum BExpr {
    Value(bool),
    Equal(Box<AExpr>, Box<AExpr>),
    Less(Box<AExpr>, Box<AExpr>),
    And(Box<BExpr>, Box<BExpr>),
    Or(Box<BExpr>, Box<BExpr>),
    Not(Box<BExpr>),
}
type BoxedBExpr = Box<BExpr>;

pub static TRUE : BExpr = BExpr::Value(true);
pub static FALSE : BExpr = BExpr::Value(false);
