use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use crate::while_lang::types::State;
use super::aexpr::AExpr;
use super::bexpr::BExpr;

#[derive(Debug)]
pub enum Cmd {
    Skip,
    Assign(Variable, Box<AExpr>),
    Seq(Box<Cmd>, Box<Cmd>),
    If(Box<BExpr>, Box<Cmd>, Box<Cmd>),
    While(Box<BExpr>, Box<Cmd>),
}
pub fn evalStep (ast: Box<Cmd>, state: &mut State) -> Result<Box<Cmd>, String> {
    use super::aexpr;
    use super::bexpr;
    return match *ast {
        Cmd::Skip => Ok(ast),
        Cmd::Assign(v, expr) => match *expr {
            AExpr::Value(a) => {
                state.set(v, a);
                Ok(Box::new(Cmd::Skip))
            }
        },
        Cmd::Seq(a, b) => match *a {
            AExpr::Skip => Ok(b),
            _ => match evalStep(a, state) {
                Ok(a) => Ok(Box::new(Cmd::Seq(a, b))),
                err => err
            }
        },
        Cmd::If(cond, a, b) => match *cond {
            BExpr::Value(cond) => Ok(if cond { a } else { b }),
            _ => match bexpr::evalStep(cond, state) {
                Ok(cond) => Ok(Box::new(Cmd::If(cond, a, b))),
                Err(msg) => Err(msg)
            }
        },
        Cmd::While(cond, cmd) => match *cond {
            BExpr::Value(cond) => {
                if (cond) {
                    // THIS WILL FAIL!
                    // need the ORIGINAL ast, not the evaluated one!
                    Ok(Box::new(Cmd::Seq(cmd, ast)))
                } else {
                    Ok(Box::new(Cmd::Skip))
                }
            },
            _ => match bexpr::evalStep(cond, state) {
                Ok(cond) => Ok(Box::new(Cmd::While(cond, a, b)))
                Err(err) => Err(err)
            }
        },
    }
}
