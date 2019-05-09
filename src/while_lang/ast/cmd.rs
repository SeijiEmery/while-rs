//use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use crate::while_lang::types::State;
use super::aexpr::AExpr;
use super::bexpr::BExpr;
use super::aexpr;
use super::bexpr;
use std::rc::Rc;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Cmd {
    Skip,
    Assign(Variable, Rc<AExpr>),
    Seq(Rc<Cmd>, Rc<Cmd>),
    If(Rc<BExpr>, Rc<Cmd>, Rc<Cmd>),
    While(Rc<BExpr>, Rc<BExpr>, Rc<Cmd>),
}
static SKIP : Rc<Cmd> = Rc::new(Cmd::Skip);

pub fn skip () -> Rc<Cmd> { SKIP.clone() }
pub fn assign(v: &str, a: Rc<AExpr>) -> Rc<Cmd> {
    Rc::new(Cmd::Assign(v.to_string(), a))
}
pub fn if_ (cond: Rc<BExpr>, a: Rc<Cmd>, b: Rc<Cmd>) -> Rc<Cmd> {
    Rc::new(Cmd::If(cond, a, b))
}
pub fn while_ (cond: Rc<BExpr>, body: Rc<Cmd>) -> Rc<Cmd> {
    Rc::new(Cmd::While(cond, cond.clone(), body))
}

pub fn evalStep (ast: &mut Rc<Cmd>, state: &mut State, result: &mut Result<bool, String>) -> bool {
    return match **ast {
        Cmd::Skip => false,
        Cmd::Assign(ref v, ref mut expr) => match **expr {
            AExpr::Value(ref a) => {
                state.set(v, *a);
                *ast = skip();
                true
            },
            _ => aexpr::evalStep(expr, state, result)
        },
        Cmd::Seq(ref mut a, ref mut b) => match **a {
            Cmd::Skip => { *ast = b.clone(); true },
            _ => evalStep(a, state, result)
        },
        Cmd::If(ref mut cond, ref mut a, ref mut b) => match **cond {
            BExpr::Value(true) => { *ast = a.clone(); true },
            BExpr::Value(false) => { *ast = b.clone(); true },
            _ => bexpr::evalStep(cond, state, result)
        },
        Cmd::While(ref mut cond, ref mut c0, ref mut body) => match **cond {
            BExpr::Value(false) => { *ast = skip(); true },
            BExpr::Value(true) => {
                cond.clone_from(c0);
                *ast = Rc::new(Cmd::Seq(body.clone(), ast.clone()));
                true
            },
            _ => bexpr::evalStep(cond, state, result)
        }
    }
}





//pub fn evalStep (ast: Rc<Cmd>, state: &mut State) -> Result<Rc<Cmd>, String> {
//    use super::aexpr;
//    use super::bexpr;
//    return match *ast {
//        Cmd::Skip => Ok(ast),
//        Cmd::Assign(ref v, ref expr) => match *expr {
//            AExpr::Value(ref a) => {
//                state.set(v, *a);
//                Ok(Rc::new(Cmd::Skip))
//            },
//            _ => match bexpr::evalStep(*expr, state) {
//                Ok(a) => Ok(Rc::new(Cmd::Assign(v, Rc::clone(expr)))),
//            }
//
//
//                Ok(Rc::new(AExpr::Assign()))
//        },
//        Cmd::Seq(ref a, ref b) => match *a {
//            AExpr::Skip => Ok(Rc::clone(b)),
//            _ => match evalStep(*a, state) {
//                Ok(a) => Ok(Rc::new(Cmd::Seq(a, Rc::clone(b)))),
//                err => err
//            }
//        },
//        Cmd::If(ref cond, ref a, ref b) => match *cond {
//            BExpr::Value(true) => Ok(a),
//            BExpr::Value(false) => Ok(b),
//            _ => match bexpr::evalStep(cond, state) {
//                Ok(cond) => Ok(Rc::new(Cmd::If(cond, a, b))),
//                Err(msg) => Err(msg)
//            }
//        },
//        Cmd::While(ref cond, ref cmd) => match *cond {
//            BExpr::Value(false) => Ok(Rc::new(Cmd::Skip)),
//            BExpr::Value(true) => Ok(Rc::new(Cmd::Seq(cmd, ast))),
//            _ => match bexpr::evalStep(cond, state) {
//                Ok(res) => Ok(Rc::new(Cmd::WhileEv(res, cond, cmd))),
//                Err(err) => Err(err),
//            }
//        }
//        Cmd::WhileEv(ref cond, ref cmd, ref cont) => match *cond {
//            BExpr::Value(false) => Ok(Rc::new(Cmd::Skip)),
//
//
//
//            BExpr::Value(ref cond) => {
//                if (cond) {
//                    // THIS WILL FAIL!
//                    // need the ORIGINAL ast, not the evaluated one!
//                    Ok(Rc::new(Cmd::Seq(cmd, cont)))
//                } else {
//                    Ok(Rc::new(Cmd::Skip))
//                }
//            },
//            _ => match bexpr::evalStep(cond, state) {
//                Ok(cond) => Ok(Rc::new(Cmd::While(cond, a, b)))
//                Err(err) => Err(err)
//            }
//        }
//    }
//}
