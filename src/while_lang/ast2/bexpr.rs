use std::rc::Rc;
use super::expr::Expr;
use super::state::{ State };
use super::aexpr::{ ARef };
use super::aexpr;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BExpr {
    BTrue,
    BFalse,
    Not(BRef),
    Or(BRef, BRef),
    And(BRef, BRef),
    Less(ARef, ARef),
    Equal(ARef, ARef),
}
pub type BRef = Rc<BExpr>;
pub type BResult = Result<BRef, String>;

pub fn btrue () -> BRef { Rc::new(BExpr::BTrue) }
pub fn bfalse () -> BRef { Rc::new(BExpr::BFalse) }
pub fn not (a: BRef) -> BRef { Rc::new(BExpr::Not(a)) }
pub fn or (a: BRef, b: BRef) -> BRef { Rc::new(BExpr::Or(a, b)) }
pub fn and (a: BRef, b: BRef) -> BRef { Rc::new(BExpr::And(a, b)) }
pub fn less (a: ARef, b: ARef) -> BRef { Rc::new(BExpr::Less(a, b)) }
pub fn equal (a: ARef, b: ARef) -> BRef { Rc::new(BExpr::Equal(a, b)) }

impl Expr<bool, BRef> for BRef {
    fn is_reduced (&self) -> bool {
        return match **self {
            BExpr::BTrue => true,
            BExpr::BFalse => true,
            _ => false
        }
    }
    fn eval (&self, state: &State) -> Result<bool, String> {
        Err("unimplemented!".to_string())
    }
    fn eval1 (&self, state: &State) -> BResult {
        Err("unimplemented!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::state_mocks::*;

    // Tests TBD
}
