use std::rc::Rc;
use super::expr::Expr;
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

impl Expr<bool, CRef> for CRef {
    fn is_reduced (&self) -> bool {
        return match **self {
            Cmd::Skip => true,
            _ => false
        }
    }
    fn eval (&self, state: &State) -> Result<bool, String> {
        Err("unimplemented!".to_string())
    }
    fn eval1 (&self, state: &State) -> CResult {
        Err("unimplemented!".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::state_mocks::*;

    // Tests TBD
}
