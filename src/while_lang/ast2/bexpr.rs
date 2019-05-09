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
    use super::super::*;

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
    #[test]
    fn test_less () {
        let state = MockEmptyState::new();
        assert_eq!(false, less(val(10), val(10)).is_reduced());
        assert_eq!(true, less(val(10), val(10)).eval1(&state).unwrap().is_reduced());
        assert_eq!(Ok(btrue()), less(val(9), val(10)).eval1(&state));
        assert_eq!(Ok(bfalse()), less(val(10), val(10)).eval1(&state));
        assert_eq!(Ok(bfalse()), less(val(10), val(9)).eval1(&state));
    }
    #[test]
    fn test_equal () {
        let state = MockEmptyState::new();
        assert_eq!(false, equal(val(10), val(10)).is_reduced());
        assert_eq!(true, equal(val(10), val(10)).eval1(&state).unwrap().is_reduced());
        assert_eq!(Ok(bfalse()), equal(val(9), val(10)).eval1(&state));
        assert_eq!(Ok(btrue()), equal(val(10), val(10)).eval1(&state));
        assert_eq!(Ok(bfalse()), equal(val(10), val(9)).eval1(&state));
    }
    #[test]
    fn test_compound () {
        let state = MockStateWithVar::new("x", 10);
        let a0 = and(
            less(val(9), var("x")),
            or(not(equal(val(10), var("x"))),
                    not(not(and(bfalse(), less(val(9), var("x")))))));
        let a1 = and(
            less(val(9), val(10)),
            or(not(equal(val(10), var("x"))),
                    not(not(and(bfalse(), less(val(9), var("x")))))));
        let a2 = and(
            btrue(),
            or(not(equal(val(10), var("x"))),
                    not(not(and(bfalse(), less(val(9), var("x")))))));
        let a3 = and(
            btrue(),
            or(not(equal(val(10), val(10))),
                    not(not(and(bfalse(), less(val(9), var("x")))))));
        let a4 = and(
            btrue(),
            or(not(btrue()),
                    not(not(and(bfalse(), less(val(9), var("x")))))));
        let a5 = and(
            btrue(),
            or(bfalse(),
               not(not(and(bfalse(), less(val(9), var("x")))))));
        let a6 = and(
            btrue(),
            or(bfalse(),
               not(not(bfalse()))));
        let a7 = and(
            btrue(),
            or(bfalse(),
               not(btrue())));
        let a8 = and(
            btrue(),
            or(bfalse(),
               bfalse()));
        let a9 = and(
            btrue(),
            bfalse());
        let a10 = bfalse();
        let a11 = bfalse();

        assert_eq!(Ok(a1.clone()), a0.eval1(&state));
        assert_eq!(Ok(a2.clone()), a1.eval1(&state));
        assert_eq!(Ok(a3.clone()), a2.eval1(&state));
        assert_eq!(Ok(a4.clone()), a3.eval1(&state));
        assert_eq!(Ok(a5.clone()), a4.eval1(&state));
        assert_eq!(Ok(a6.clone()), a5.eval1(&state));
        assert_eq!(Ok(a7.clone()), a6.eval1(&state));
        assert_eq!(Ok(a8.clone()), a7.eval1(&state));
        assert_eq!(Ok(a9.clone()), a8.eval1(&state));
        assert_eq!(Ok(a10.clone()), a9.eval1(&state));
        assert_eq!(Ok(a11.clone()), a10.eval1(&state));
        assert_eq!(Ok(a11.clone()), a11.eval1(&state));
    }
    #[test]
    fn test_compound_errors () {
        let state = MockStateWithVar::new("x", 10);
        let a0 = and(
            less(val(9), var("x")),
            or(not(equal(val(10), var("y"))),
                    not(not(and(bfalse(), less(val(9), var("x")))))));
        let a1 = and(
            less(val(9), val(10)),
            or(not(equal(val(10), var("y"))),
                    not(not(and(bfalse(), less(val(9), var("x")))))));
        let a2 = and(
            btrue(),
            or(not(equal(val(10), var("y"))),
                    not(not(and(bfalse(), less(val(9), var("x")))))));

        assert_eq!(Ok(a1.clone()), a0.eval1(&state));
        assert_eq!(Ok(a2.clone()), a1.eval1(&state));
        assert_eq!(true, a2.eval1(&state).is_err());
    }
}
