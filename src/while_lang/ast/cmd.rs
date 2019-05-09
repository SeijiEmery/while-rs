//use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use crate::while_lang::types::State;
use super::aexpr::AExpr;
use super::aexpr::ARef;
use super::bexpr::BExpr;
use super::bexpr::BRef;
use super::aexpr;
use super::bexpr;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Cmd {
    Skip,
    Assign(Variable, ARef),
    Seq(CRef, CRef),
    If(BRef, CRef, CRef),
    While(BRef, BRef, CRef),
}
pub type CRef = Rc<RefCell<Cmd>>;

static SKIP : CRef = Rc::new(RefCell::new(Cmd::Skip));

pub fn skip () -> CRef { SKIP.clone() }
pub fn seq (a: CRef, b: CRef) -> CRef {
    Rc::new(RefCell::new(Cmd::Seq(a, b)))
}
pub fn assign(v: &str, a: ARef) -> CRef {
    Rc::new(RefCell::new(Cmd::Assign(v.to_string(), a)))
}
pub fn if_ (cond: BRef, a: CRef, b: CRef) -> CRef {
    Rc::new(RefCell::new(Cmd::If(cond, a, b)))
}
pub fn while_ (cond: BRef, body: CRef) -> CRef {
    Rc::new(RefCell::new(Cmd::While(cond, cond.clone(), body)))
}

// lazy eval (single step)
pub fn evalStep (ast: &mut CRef, state: &mut State, result: &mut Result<bool, String>) -> bool {
    return match *(ast.borrow_mut()) {
        Cmd::Skip => false,
        Cmd::Assign(ref v, ref mut expr) => match *(expr.borrow_mut()) {
            AExpr::Value(ref a) => {
                state.set(v, *a);
                *ast = skip();
                true
            },
            _ => aexpr::evalStep(expr, state, result)
        },
        Cmd::Seq(ref mut a, ref mut b) => match *(a.borrow_mut()) {
            Cmd::Skip => { *ast = b.clone(); true },
            _ => evalStep(a, state, result)
        },
        Cmd::If(ref mut cond, ref mut a, ref mut b) => match *(cond.borrow_mut()) {
            BExpr::Value(true) => { *ast = a.clone(); true },
            BExpr::Value(false) => { *ast = b.clone(); true },
            _ => bexpr::evalStep(cond, state, result)
        },
        Cmd::While(ref mut cond, ref mut c0, ref mut body) => match *(cond.borrow_mut()) {
            BExpr::Value(false) => { *ast = skip(); true },
            BExpr::Value(true) => {
                cond.clone_from(c0);
                *ast = seq(body.clone(), ast.clone());
                true
            },
            _ => bexpr::evalStep(cond, state, result)
        }
    }
}
