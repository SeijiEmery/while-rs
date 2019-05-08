use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use crate::while_lang::types::State;
use super::aexpr::AExpr;
use super::bexpr::BExpr;

#[derive(Debug)]Skip,
    Assign(Variable, Box<AExpr>),
    Seq(Box<Cmd>, Box<Cmd>),
    If(Box<BExpr>, Box<Cmd>, Box<Cmd>),
    While(Box<BExpr>, Box<Cmd>),
}

pub fn evalStep (ast: Box<Cmd>, state: &mut State) -> Result<Box<Cmd>, String> {
    use super::aexpr;
    use super::bexpr;
    return match *ast {
        Cmd::Assign(v, expr) =>
    }
}







