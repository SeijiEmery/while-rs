mod expr;
mod state;
mod state_mocks;
mod aexpr;
mod bexpr;
mod cmd;

use expr::{ Expr };
use state::{ State, Value, Variable, VResult };
use aexpr::{ ARef, AResult, val, var, add, sub, mul };
use bexpr::{ BRef, BResult, btrue, bfalse, not, or, and, less, equal };
use cmd::{ CRef, CResult, skip, seq, assign, if_, while_ };

