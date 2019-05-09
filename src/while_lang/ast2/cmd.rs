use std::rc::Rc;
use super::expr::{ CmdExpr, Expr };
use super::state::{ State, Variable };
use super::aexpr::{ ARef, AExpr };
use super::bexpr::{ BRef, BExpr };
use super::aexpr;
use super::bexpr;

#[derive(Debug, PartialOrd, PartialEq)]
enum Cmd {
    Skip,
    Seq(CRef, CRef),
    Assign(Variable, ARef),
    If(BRef, CRef, CRef),
    While(BRef, BRef, CRef),
}
pub type CRef = Rc<Cmd>;
pub type CResult = Result<CRef, String>;

pub fn skip () -> CRef { Rc::new(Cmd::Skip) }
pub fn seq (a: CRef, b: CRef) -> CRef { Rc::new(Cmd::Seq(a, b)) }
pub fn assign (v: &str, expr: ARef) -> CRef { Rc::new(Cmd::Assign(v.to_string(), expr)) }
pub fn if_ (cond: BRef, a: CRef, b: CRef) -> CRef { Rc::new(Cmd::If(cond, a, b)) }
pub fn while_ (cond: BRef, body: CRef) -> CRef { Rc::new(Cmd::While(cond.clone(), cond, body)) }
fn while_ev (cond: BRef, c0: BRef, body: CRef) -> CRef { Rc::new(Cmd::While(cond, c0, body)) }

impl CmdExpr<CRef> for CRef {
    fn is_reduced (&self) -> bool {
        return match **self {
            Cmd::Skip => true,
            _ => false
        }
    }
    // for reference
    fn eval (&self, state: &mut State) -> Result<(), String> {
        match **self {
            Cmd::Skip => Ok(()),
            Cmd::Seq(ref a, ref b) => match a.eval(state) {
                Ok(_) => match b.eval(state) {
                    Ok(_) => Ok(()),
                    err => err
                }, err => err
            },
            Cmd::Assign(ref v, ref expr) => match expr.eval(state) {
                Ok(a) => { state.set(v, a); Ok(()) }
                Err(msg) => Err(msg)
            },
            Cmd::If(ref cond, ref a, ref b) => match cond.eval(state) {
                Ok(true) => a.eval(state),
                Ok(false) => b.eval(state),
                Err(msg) => Err(msg)
            },
            Cmd::While(ref cond, ref c0, ref body) => {
                loop { // dangerous: an infinite loop will loop infinitely!
                    match cond.eval(state) {
                        Ok(true) => { body.eval(state); },
                        Ok(false) => { return Ok(()) },
                        Err(msg) => { return Err(msg) },
                    }
                }
            }
        }
    }
    fn eval1 (&self, state: &mut State) -> CResult {
        match **self {
            Cmd::Skip => Ok(self.clone()),
            Cmd::Seq(ref a, ref b) => match **a {
                Cmd::Skip => b.eval1(state),
                _ => a.eval1(state).map(|a| seq(a, b.clone())),
            },
            Cmd::Assign(ref v, ref expr) => match **expr {
                AExpr::Val( a) => { state.set(v, a); Ok(skip()) },
                _ => expr.eval1(state).map(|expr| assign(&v, expr)),
            },
            Cmd::If(ref cond, ref a, ref b) => match **cond {
                BExpr::BTrue => Ok(a.clone()),
                BExpr::BFalse => Ok(b.clone()),
                _ => cond.eval1(state).map(|cond| if_(cond, a.clone(), b.clone()))
            },
            Cmd::While(ref cond, ref c0, ref body) => match **cond {
                BExpr::BFalse => Ok(skip()),
                BExpr::BTrue => Ok(seq(body.clone(), while_(c0.clone(), body.clone()))),
                _ => cond.eval1(state).map(|cond| while_ev(cond, c0.clone(), body.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::state::*;
    use super::super::*;

    #[test]
    fn test_skip () {
        let mut state = HashState::new();
        assert_eq!(true, skip().is_reduced());
        assert_eq!(Ok(skip()), skip().eval1(&mut state));
        assert_eq!(true, skip().eval1(&mut state).unwrap().is_reduced());
        assert_eq!(Ok(skip()), skip().eval1(&mut state).unwrap().eval1(&mut state));
    }
    #[test]
    fn test_assign () {
        let mut state = HashState::new();
        let a = assign("x", val(10));
        let a0 = assign("x", val(10));
        let a1 = skip();

        assert_eq!(false, a.is_reduced());
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(a, a0);

        let b = a.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b);
        assert_eq!(false, a.is_reduced());
        assert_eq!(true, b.is_reduced());
        assert_eq!(false, state.is_empty());
        assert_eq!(Ok(10), state.get("x"));
    }
    #[test]
    fn test_seq () {
        let mut state = HashState::new();
        let a = seq(assign("x", val(10)), assign("x", val(20)));
        let a0 = seq(assign("x", val(10)), assign("x", val(20)));
        let a1 = seq(skip(), assign("x", val(20)));
        let a2 = skip();

        assert_eq!(false, a.is_reduced());
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(a, a0);

        let b1 = a.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(false, a.is_reduced());
        assert_eq!(false, b1.is_reduced());
        assert_eq!(Ok(10), state.get("x"));

        let b2 = b1.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(a2, b2);
        assert_eq!(true, b2.is_reduced());
        assert_eq!(Ok(20), state.get("x"));

        let mut empty_state = HashState::new();
        let b3 = b2.eval1(&mut empty_state).unwrap();
        assert_eq!(b2, b3);
        assert_eq!(Ok(20), state.get("x"));
        assert_eq!(true, empty_state.get("x").is_err());
    }
    #[test]
    fn test_if_true () {
        let mut state = HashState::new();
        let a = if_(btrue(), assign("x", val(10)), assign("y", val(10)));
        let a0 = if_(btrue(), assign("x", val(10)), assign("y", val(10)));
        let a1 = assign("x", val(10));
        let a2 = skip();

        assert_eq!(false, a.is_reduced());
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(true, state.get("y").is_err());
        assert_eq!(a, a0);

        let b1 = a.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(true, state.get("y").is_err());

        let b2 = b1.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(a2, b2);
        assert_eq!(Ok(10), state.get("x"));
        assert_eq!(true, state.get("y").is_err());

        let b3 = b2.eval1(&mut state).unwrap();
        assert_eq!(b2, b3);
        assert_eq!(Ok(10), state.get("x"));
        assert_eq!(true, state.get("y").is_err());
    }
    #[test]
    fn test_if_false () {
        let mut state = HashState::new();
        let a = if_(bfalse(), assign("x", val(10)), assign("y", val(10)));
        let a0 = if_(bfalse(), assign("x", val(10)), assign("y", val(10)));
        let a1 = assign("y", val(10));
        let a2 = skip();

        assert_eq!(false, a.is_reduced());
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(true, state.get("y").is_err());
        assert_eq!(a, a0);

        let b1 = a.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(true, state.get("y").is_err());

        let b2 = b1.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(a2, b2);
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(Ok(10), state.get("y"));

        let b3 = b2.eval1(&mut state).unwrap();
        assert_eq!(b2, b3);
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(Ok(10), state.get("y"));
    }
    #[test]
    fn test_while_false () {
        let mut state = HashState::new();
        let a = while_(bfalse(), assign("x", val(10)));
        let a0 = while_(bfalse(), assign("x", val(10)));
        let a1 = skip();

        assert_eq!(false, a.is_reduced());
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(a, a0);

        let b1 = a.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(true, b1.is_reduced());
        assert_eq!(true, state.get("x").is_err());

        let b2 = b1.eval1(&mut state).unwrap();
        assert_eq!(b1, b2);
        assert_eq!(true, b2.is_reduced());
        assert_eq!(true, state.get("x").is_err());
    }
    #[test]
    fn test_while_true () {
        let mut state = HashState::new();
        let a = while_(btrue(), assign("x", val(10)));
        let a0 = while_(btrue(), assign("x", val(10)));
        let a1 = seq(assign("x", val(10)), while_(btrue(), assign("x", val(10))));
        let a2 = seq(skip(), while_(btrue(), assign("x", val(10))));
        let a3 = seq(assign("x", val(10)), while_(btrue(), assign("x", val(10))));

        assert_eq!(false, a.is_reduced());
        assert_eq!(true, state.get("x").is_err());
        assert_eq!(a, a0);

        let b1 = a.eval1(&mut state).unwrap();
        assert_eq!(a0, a);
        assert_eq!(a1, b1);
        assert_eq!(false, b1.is_reduced());
        assert_eq!(true, state.get("x").is_err());

        let b2 = b1.eval1(&mut state).unwrap();
        assert_eq!(a2, b2);
        assert_eq!(false, b2.is_reduced());
        assert_eq!(Ok(10), state.get("x"));

        state.set("x", 20);
        assert_eq!(Ok(20), state.get("x"));

        let b3 = b2.eval1(&mut state).unwrap();
        assert_eq!(a3, b3);
        assert_eq!(Ok(20), state.get("x"));
        assert_eq!(b1, b3);

        let b4 = b3.eval1(&mut state).unwrap();
        assert_eq!(Ok(10), state.get("x"));
    }
}
