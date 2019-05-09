mod expr;
mod state;
mod state_mocks;
mod aexpr;

use expr::{ Expr };
use state::{ State, Value, Variable, VResult };
use aexpr::{ ARef, AResult, val, var, add, sub, mul };
