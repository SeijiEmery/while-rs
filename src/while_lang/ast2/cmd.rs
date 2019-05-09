use std::rc::Rc;
use super::expr::{ CmdExpr, Expr };
use super::state::{ State, Variable };
use super::aexpr::{ ARef };
use super::bexpr::{ BRef };
use super::aexpr;
use super::bexpr;

#[derive(Debug, PartialOrd, PartialEq)]
enum Cmd {
    Skip,
    Seq(CRef, CRef),
    Assign(Variable, ARef),
    If(BRef, CRef, CRef),
    While(BRef, CRef),
}
pub type CRef = Rc<Cmd>;
pub type CResult = Result<CRef, String>;

pub fn skip () -> CRef { Rc::new(Cmd::Skip) }
pub fn seq (a: CRef, b: CRef) -> CRef { Rc::new(Cmd::Seq(a, b)) }
pub fn assign (v: &str, expr: ARef) -> CRef { Rc::new(Cmd::Assign(v.to_string(), expr)) }
pub fn if_ (cond: BRef, a: CRef, b: CRef) -> CRef { Rc::new(Cmd::If(cond, a, b)) }
pub fn while_ (cond: BRef, body: CRef) -> CRef { Rc::new(Cmd::While(cond, body)) }

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
            Cmd::While(ref cond, ref body) => {
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
        Err("unimplemented!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::state_mocks::*;

    #[test]
    fn test_skip () {
        let state = MockEmptyState::new();
        assert_eq!(true, skip().is_reduced());
    }
}
