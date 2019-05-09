use std::fmt::format;
use std::fmt;
use std::cmp;
use std::rc::Rc;

pub type Value = i64;
pub type VResult = Result<Value, String>;
pub type AResult = Result<ARef, String>;

pub trait State {
    fn get (&self, var: &str) -> VResult;
    fn set (&mut self, var: &str, val: Value);
}
pub trait Expr <Value, AST>: fmt::Debug {
    fn eval (&self, state: &State) -> Result<Value, String>;
    fn eval1 (&self, state: &State) -> Result<AST, String>;
    fn is_reduced (&self) -> bool;
}


#[derive(Debug, PartialEq, PartialOrd)]
enum BinOp { Add, Sub, Mul }

#[derive(Debug, PartialOrd, PartialEq)]
enum AExpr {
    Val(Value),
    Var(String),
    Binary(BinOp, ARef, ARef)
}
pub type ARef = Rc<AExpr>;

// constructors
pub fn val (v: Value) -> ARef { Rc::new(AExpr::Val(v)) }
pub fn var (v: &str) -> ARef { Rc::new(AExpr::Var(v.to_string())) }
fn binop (op: BinOp, a: ARef, b: ARef) -> ARef { Rc::new(AExpr::Binary(op, a, b)) }
pub fn add (a: ARef, b: ARef) -> ARef { binop(BinOp::Add, a, b) }
pub fn sub (a: ARef, b: ARef) -> ARef { binop(BinOp::Sub, a, b) }
pub fn mul (a: ARef, b: ARef) -> ARef { binop(BinOp::Mul, a, b) }

impl Expr<Value, ARef> for ARef {

    /// Values (terminals) are reduced. Everything else is not.
    fn is_reduced (&self) -> bool {
        return match **self {
            AExpr::Val(_) => true,
            _ => false
        }
    }

    /// Eagerly evaluates an arithmetic expression.
    /// Returns Result<Value, String>, as evaluation will fail for undefined variables (given state).
    /// On failure, this returns a helpful error message, though the specifics of this are abstracted
    /// out to the State impl.
    fn eval (&self, state: &State) -> VResult {
        return match **self {
            AExpr::Val(ref x) => Ok(*x),
            AExpr::Var(ref x) => state.get(&x),
            AExpr::Binary(ref op, ref a, ref b) => match a.eval(state) {
                Ok(a) => match b.eval(state) {
                    Ok(b) => match *op {
                        BinOp::Add => Ok(a + b),
                        BinOp::Sub => Ok(a - b),
                        BinOp::Mul => Ok(a * b),
                    }, err => err
                }, err => err
            }
        }
    }

    /// Lazily evaluates a single step of an arithmetic expression.
    /// There are effectively two cases:
    /// 1) this expression has been reduced to the point where we can use eval() to run the last step
    /// 2) this expression has non-reduced subtrees
    ///
    /// 1) consists of the following subcases:
    /// 1a) AST is Value (terminal)
    /// 1b) AST is Var  (1 step to execute)
    /// 1c) AST is Binary with terminal arguments (1 step to execute)
    ///
    /// For 2) we call eval1() on the first non-reduced subtree, and reconstruct this AST node
    /// with the "updated" value. To keep the AST immutable (and thus permit shared references to
    /// past AST states), this entails creating an entirely new node with partial updates.
    fn eval1 (&self, state: &State) -> AResult {
        match **self {
            // Case 2): if we have a binary expression, check if either of the args have not yet
            // been fully reduced. If that's the case,
            AExpr::Binary(ref op, ref a, ref b) => {
                if !a.is_reduced() {
                    let mut res= self.clone();
                    return a.eval1(state).map(|a| { update_left(&mut res, a); res })
                }
                if !b.is_reduced() {
                    let mut res= self.clone();
                    return b.eval1(state).map(|b| { update_right(&mut res, b); res })
                }
            }, _ => {}
        }
        // Case 1) (terminal, ie. this is either a Val(), Var(), or Binary() with two Value()
        // arguments. As there is at most 1 eval step to do, we can execute this by calling eval()
        // and then mapping the result to an ARef using val())
        //
        // Note that Result.map() is monadic and forwards Err() values for us.
        return self.eval(state).map(|x| val(x));
    }
}
fn update_left (ast: &mut ARef, left: ARef) {
    match Rc::get_mut(ast) {
        Some(ref mut ast) => match ast {
            AExpr::Binary(ref mut op, ref mut a, ref mut b) => {
                *a = left;
            }, _ => {}
        }, _ => {}
    }
}
fn update_right (ast: &mut ARef, right: ARef) {
    match Rc::get_mut(ast) {
        Some(ref mut ast) => match ast {
            AExpr::Binary(ref mut op, ref mut a, ref mut b) => {
                *b = right;
            }, _ => {}
        }, _ => {}
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
        assert_eq!(val(10).is_reduced(), true);
        assert_eq!(val(10).eval(&state), Ok(10));
        assert_eq!(val(10).eval1(&state), Ok(val(10)));
    }
    #[test]
    fn test_var () {
        let state_empty = MockEmptyState::new();
        let state_x_10 = MockStateWithVar::new("x", 10);

        assert_eq!(false, var("x").is_reduced());
        assert_eq!(true, var("x").eval(&state_empty).is_err());
        assert_eq!(false, var("x").eval(&state_x_10).is_err());
        assert_eq!(Ok(10), var("x").eval(&state_x_10));
        assert_eq!(true, var("y").eval(&state_x_10).is_err());

        assert_eq!(true, var("x").eval1(&state_empty).is_err());
        assert_eq!(false, var("x").eval1(&state_x_10).is_err());
        assert_eq!(Ok(val(10)), var("x").eval1(&state_x_10));
        assert_eq!(true, var("y").eval1(&state_x_10).is_err());

        let res : ARef = var("x").eval1(&state_x_10).unwrap();
        assert_eq!(true, res.is_reduced());
        assert_eq!(Ok(10), res.eval(&state_empty));
        assert_eq!(Ok(res.clone()), res.eval1(&state_empty));
    }
    #[test]
    fn test_binary_eval () {
        let state_empty = MockEmptyState::new();
        let state_x_10 = MockStateWithVar::new("x", 10);

        assert_eq!(Ok(20), add(val(10), val(10)).eval(&state_empty));
        assert_eq!(Ok(0), sub(val(10), val(10)).eval(&state_empty));
        assert_eq!(Ok(100), mul(val(10), val(10)).eval(&state_empty));

        assert_eq!(Ok(20), add(var("x"), val(10)).eval(&state_x_10));
        assert_eq!(Ok(20), add(val(10), var("x")).eval(&state_x_10));
        assert_eq!(true, add(var("x"), val(10)).eval(&state_empty).is_err());
        assert_eq!(true, add(val(10), var("x")).eval(&state_empty).is_err());
    }
    #[test]
    fn min_test_binary_stepping () {
        let state = MockStateWithVar::new("x", 10);
        let empty_state = MockEmptyState::new();

        let a0 = add(sub(mul(val(2), val(6)), var("x")), val(4));
        let a1 = add(sub(val(12), var("x")), val(4));
        let a2 = add(sub(val(12), val(10)), val(4));
        let a3 = add(val(2), val(4));
        let a4 = val(6);

        assert_eq!(Ok(a4.clone()), a4.eval1(&state));
        assert_eq!(Ok(a4.clone()), a3.eval1(&state));
        assert_eq!(Ok(a3.clone()), a2.eval1(&state));
        assert_eq!(Ok(a2.clone()), a1.eval1(&state));
        assert_eq!(Ok(a1.clone()), a0.eval1(&state));

        assert_eq!(false, a4.eval1(&state).is_err());
        assert_eq!(false, a3.eval1(&state).is_err());
        assert_eq!(false, a2.eval1(&state).is_err());
        assert_eq!(true, a1.eval1(&state).is_err());
        assert_eq!(false, a0.eval1(&state).is_err());
    }
    #[test]
    fn test_binary_stepping () {
        let state = MockStateWithVar::new("x", 10);
        let a = add(sub(mul(val(2), val(6)), var("x")), val(4));
        let a0 = add(sub(mul(val(2), val(6)), var("x")), val(4));
        let a1 = add(sub(val(12), var("x")), val(4));
        let a2 = add(sub(val(12), val(10)), val(4));
        let a3 = add(val(2), val(4));
        let a4 = val(6);

        assert_eq!(a.is_reduced(), false);
        assert_eq!(a0, a);
        assert_eq!(Ok(6), a.eval(&state));
        assert_eq!(a0, a);

        let b = a.eval1(&state).unwrap();
        assert_eq!(false, b.is_reduced());
        assert_eq!(Ok(6), a.eval(&state));
        assert_eq!(a0, a);
        assert_eq!(Ok(6), b.eval(&state));
        assert_eq!(a1, b);

        let c = a.eval1(&state).unwrap();
        assert_eq!(c, b);

        let b2 = b.eval1(&state).unwrap();
        assert_eq!(false, b2.is_reduced());
        assert_eq!(Ok(6), b2.eval(&state));
        assert_eq!(a0, a);
        assert_eq!(a1, b);
        assert_eq!(a2, b2);

        let b3 = b2.eval1(&state).unwrap();
        assert_eq!(false, b3.is_reduced());
        assert_eq!(Ok(6), b3.eval(&state));
        assert_eq!(a0, a);
        assert_eq!(a1, b);
        assert_eq!(a2, b2);
        assert_eq!(a3, b3);

        let b4 = b3.eval1(&state).unwrap();
        assert_eq!(true, b4.is_reduced());
        assert_eq!(Ok(6), b3.eval(&state));
        assert_eq!(a0, a);
        assert_eq!(a1, b);
        assert_eq!(a2, b2);
        assert_eq!(a3, b3);
        assert_eq!(a4, b4);

        let b5 = b4.eval1(&state).unwrap();
        assert_eq!(Ok(6), b5.eval(&state));
        assert_eq!(b4, b5);

        let mut ast = a;
        assert_eq!(true, ast.is_reduced());
        while (!ast.is_reduced()) {
            ast = ast.eval1(&state).unwrap();
            assert_eq!(Ok(6), ast.eval(&state));
        }
    }
}
