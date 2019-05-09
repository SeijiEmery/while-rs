use std::rc::Rc;
use super::expr::Expr;
use super::state::{ State };
use super::aexpr::{ ARef, AExpr };
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
        match **self {
            BExpr::BTrue => Ok(self.clone()),
            BExpr::BFalse => Ok(self.clone()),
            BExpr::Not(ref a) => match **a {
                BExpr::BTrue => Ok(bfalse()),
                BExpr::BFalse => Ok(btrue()),
                _ => a.eval1(state).map(|a| not(a))
            },
            BExpr::Or(ref a, ref b) => match **a {
                BExpr::BTrue => Ok(a.clone()),
                BExpr::BFalse => match **b {
                    BExpr::BTrue => Ok(b.clone()),
                    BExpr::BFalse => Ok(b.clone()),
                    _ => b.eval1(state).map(|b| or(a.clone(), b))
                }, _ => a.eval1(state).map(|a| or(a, b.clone()))
            },
            BExpr::And(ref a, ref b) => match **a {
                BExpr::BFalse => Ok(a.clone()),
                BExpr::BTrue => match **b {
                    BExpr::BTrue => Ok(b.clone()),
                    BExpr::BFalse => Ok(b.clone()),
                    _ => b.eval1(state).map(|b| and(a.clone(), b))
                }, _ => a.eval1(state).map(|a| and(a, b.clone()))
            },
            BExpr::Less(ref a, ref b) => match **a {
                AExpr::Val(xa) => match **b {
                    AExpr::Val(xb) => match xa < xb {
                        true => Ok(btrue()),
                        false => Ok(bfalse()),
                    }, _ => b.eval1(state).map(|b| less(a.clone(), b))
                }, _ => a.eval1(state).map(|a| less(a, b.clone()))
            },
            BExpr::Equal(ref a, ref b) => match **a {
                AExpr::Val(xa) => match **b {
                    AExpr::Val(xb) => match xa == xb {
                        true => Ok(btrue()),
                        false => Ok(bfalse()),
                    }, _ => b.eval1(state).map(|b| equal(a.clone(), b))
                }, _ => a.eval1(state).map(|a| equal(a, b.clone()))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::state_mocks::*;

    #[test]
    fn test_true () {
        let state = MockEmptyState::new();
        assert_eq!(true, btrue().is_reduced());
        assert_eq!(Ok(btrue()), btrue().eval1(&state));
        assert_eq!(true, btrue().eval1(&state).unwrap().is_reduced());
        assert_eq!(Ok(btrue()), btrue().eval1(&state).unwrap().eval1(&state));
    }
    #[test]
    fn test_false () {
        let state = MockEmptyState::new();
        assert_eq!(true, bfalse().is_reduced());
        assert_eq!(Ok(bfalse()), bfalse().eval1(&state));
        assert_eq!(true, bfalse().eval1(&state).unwrap().is_reduced());
        assert_eq!(Ok(bfalse()), bfalse().eval1(&state).unwrap().eval1(&state));
    }
    #[test]
    fn test_not () {
        let state = MockEmptyState::new();
        assert_eq!(false, not(btrue()).is_reduced());
        assert_eq!(false, not(bfalse()).is_reduced());
        assert_eq!(true, not(btrue()).eval1(&state).unwrap().is_reduced());
        assert_eq!(true, not(bfalse()).eval1(&state).unwrap().is_reduced());
        assert_eq!(Ok(bfalse()), not(btrue()).eval1(&state));
        assert_eq!(Ok(btrue()), not(bfalse()).eval1(&state));
    }
    #[test]
    fn test_or () {
        let state = MockEmptyState::new();
        assert_eq!(false, or(btrue(), btrue()).is_reduced());
        assert_eq!(true, or(btrue(), btrue()).eval1(&state).unwrap().is_reduced());
        assert_eq!(Ok(btrue()), or(btrue(), btrue()).eval1(&state));
        assert_eq!(Ok(btrue()), or(bfalse(), btrue()).eval1(&state));
        assert_eq!(Ok(btrue()), or(btrue(), bfalse()).eval1(&state));
        assert_eq!(Ok(bfalse()), or(bfalse(), bfalse()).eval1(&state));
    }
    #[test]
    fn test_and () {
        let state = MockEmptyState::new();
        assert_eq!(false, and(btrue(), btrue()).is_reduced());
        assert_eq!(true, and(btrue(), btrue()).eval1(&state).unwrap().is_reduced());
        assert_eq!(Ok(btrue()), and(btrue(), btrue()).eval1(&state));
        assert_eq!(Ok(bfalse()), and(bfalse(), btrue()).eval1(&state));
        assert_eq!(Ok(bfalse()), and(btrue(), bfalse()).eval1(&state));
        assert_eq!(Ok(bfalse()), and(bfalse(), bfalse()).eval1(&state));
    }
}
