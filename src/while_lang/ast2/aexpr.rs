use std::fmt::format;
use std::fmt;
use std::cmp;

pub type Value = i64;
pub type VResult = Result<Value, String>;
pub type AResult = Result<ARef, String>;
pub type ARef = Box<AExpr>;

pub trait State {
    fn get (&self, var: &str) -> VResult;
    fn set (&mut self, var: &str, val: Value);
}
pub trait AExpr: fmt::Debug + PartialEq {
    fn eval (&self, state: &State) -> VResult;
    fn eval1 (&self, state: &State) -> AResult;
    fn is_reduced (&self) -> bool;
}
impl PartialEq<ARef> for ARef {
    fn eq (&self, other: &ARef) -> bool {
        **self == **other
    }
}

//impl fmt::Debug for ARef {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        (**self).fmt(f)
//    }
//}



#[derive(Debug, PartialEq)]
struct Val(Value);
pub fn val (v: Value) -> ARef { Box::new(Val(v)) }
impl AExpr for Val {
    fn eval (&self, state: &State) -> VResult { Ok(self.0) }
    fn eval1 (&self, state: &State) -> AResult { Ok(val(self.0)) }
    fn is_reduced (&self) -> bool { true }
}
//impl PartialEq<Val> for Val {
//    fn eq (&self, other: &Val) -> bool { self.0 == other.0 }
//}

#[derive(Debug, PartialEq)]
struct Var(String);
pub fn var (v: &str) -> ARef { Box::new(Var(v.to_string())) }
impl AExpr for Var {
    fn eval (&self, state: &State) -> VResult { state.get(&self.0) }
    fn eval1 (&self, state: &State) -> AResult { state.get(&self.0).map(|a| val(a)) }
    fn is_reduced (&self) -> bool { false }
}
//impl PartialEq<Var> for Var {
//    fn eq (&self, other: &Var) -> bool { self.0 == other.0 }
//}

#[derive(Debug, PartialEq)]
enum BinOp { Add, Sub, Mul }

#[derive(Debug, PartialEq)]
struct ABinary(BinOp, ARef, ARef);
fn binop (op: BinOp, a: ARef, b: ARef) -> ARef {
    Box::new(ABinary(op, a, b))
}
//impl PartialEq<ABinary> for ABinary {
//    fn eq (&self, other: &ABinary) -> bool {
//        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
//    }
//}
pub fn add (a: ARef, b: ARef) -> ARef { binop(BinOp::Add, a, b) }
pub fn sub (a: ARef, b: ARef) -> ARef { binop(BinOp::Sub, a, b) }
pub fn mul (a: ARef, b: ARef) -> ARef { binop(BinOp::Mul, a, b) }

impl AExpr for ABinary {
    fn is_reduced (&self) -> bool { false }
    fn eval (&self, state: &State) -> VResult {
        match self.1.eval(state) {
            Ok(a) => match self.2.eval(state) {
                Ok(b) => match self.0 {
                    BinOp::Add => Ok(a + b),
                    BinOp::Sub => Ok(a - b),
                    BinOp::Mul => Ok(a * b),
                }, err => err
            }, err => err
        }
    }
    fn eval1 (&self, state: &State) -> AResult {
        if !self.1.is_reduced() {
            self.1.eval1(state).map(|r| binop(self.0, r, self.2))
        }
        else if !self.2.is_reduced() {
            self.2.eval1(state).map(|r| binop(self.0, self.1, r))
        }
        else {
            self.eval(state).map(val)
        }
    }
}

#[derive(Debug, PartialEq)]
struct MockEmptyState();
impl MockEmptyState { fn new () -> MockEmptyState { MockEmptyState() } }
impl State for MockEmptyState {
    fn get (&self, var: &str) -> VResult { Err(format!("undefined variable '{}'!", var)) }
    fn set (&mut self, var: &str, val: Value) {}
}

#[derive(Debug, PartialEq)]
struct MockStateWithVar { var: String, val: Value }
impl MockStateWithVar {
    fn new (var: &str, val: Value) -> MockStateWithVar {
        MockStateWithVar { var: var.to_string(), val }
    }
}
impl State for MockStateWithVar {
    fn get (&self, var: &str) -> VResult {
        if var == self.var {
            Ok(self.val)
        } else {
            Err(format!("undefined variable '{}'!", var))
        }
    }
    fn set (&mut self, var: &str, val: Value) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_val () {
        let state = MockEmptyState::new();
        assert_eq!(val(10).is_reduced(), false);
//        assert_eq!(*val(10).eval(&state), Ok(10));
//        assert_eq!(&val(10).eval1(&state), Ok(val(10)));
    }
    #[test]
    fn test_var () {
        let state_empty = MockEmptyState::new();
        let state_x_10 = MockStateWithVar::new("x", 10);

//        assert_eq!(var("x").is_reduced(), false);
//        assert_eq!(var("x").eval(&state_empty).is_err(), true);
//        assert_eq!(var("x").eval(&state_x_10).is_err(), false);
//        assert_eq!(var("x").eval(&state_x_10), Ok(10));
//        assert_eq!(var("y").eval(&state_x_10).is_err(), true);
//
//        assert_eq!(var("x").eval1(&state_empty).is_err(), true);
//        assert_eq!(var("x").eval1(&state_x_10).is_err(), false);
//        assert_eq!(var("x").eval1(&state_x_10), Ok(val(10)));
//        assert_eq!(var("y").eval1(&state_x_10).is_err(), true);
//
//        let res = var("x").eval1(&state_x_10).unwrap();
//        assert_eq!(res.is_reduced(), true);
//        assert_eq!(res.eval(&state_empty), Ok(10));
//        assert_eq!(res.eval1(&state_empty), res);
    }
    #[test]
    fn test_add () {

    }
}
