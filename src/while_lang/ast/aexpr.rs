use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use crate::while_lang::types::State;
//use std::ops::Add;
//use std::ops::Sub;
//use std::ops::Mul;
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum AExpr {
    Value(Value),
    Variable(Variable),
    Add(ARef, ARef),
    Sub(ARef, ARef),
    Mul(ARef, ARef),
}
pub type ARef = Rc<RefCell<AExpr>>;

// constructors
pub fn val (v: Value) -> ARef { Rc::new(RefCell::new(AExpr::Value(v))) }
pub fn var (v: &str) -> ARef { Rc::new(RefCell::new(AExpr::Variable(v.to_string()))) }
pub fn add (a: ARef, b: ARef) -> ARef {Rc::new(RefCell::new(AExpr::Add(a, b))) }
pub fn sub (a: ARef, b: ARef) -> ARef {Rc::new(RefCell::new(AExpr::Sub(a, b))) }
pub fn mul (a: ARef, b: ARef) -> ARef {Rc::new(RefCell::new(AExpr::Mul(a, b))) }

// recursive eager eval
pub fn eval (ast: &ARef, state: &State) -> Result<Value, String> {
    return match *(ast.borrow_mut()) {
        AExpr::Value(v) => Ok(v),
        AExpr::Variable(ref v) => state.get(&v),
        AExpr::Add(ref a, ref b) => evalBinary(|a, b| a + b, a, b, state),
        AExpr::Sub(ref a, ref b) => evalBinary(|a, b| a - b, a, b, state),
        AExpr::Mul(ref a, ref b) => evalBinary(|a, b| a * b, a, b, state),
    }
}
fn evalBinary <F>(f: F, left: &ARef, right: &ARef, state: &State) -> Result<Value, String>
    where F: FnOnce(Value, Value) -> Value
{
    return match eval(left, state) {
        Ok(a) => match eval(right, state) {
            Ok(b) => Ok(f(a, b)),
            err => err,
        },
        err => err
    };
}

// lazy single-step eval
pub fn evalStep (ast: &mut ARef, state: &State, result: &mut Result<bool, String>) -> bool {
    return match *(ast.borrow_mut()) {
        AExpr::Value(_) => false,
        AExpr::Variable(ref v) => match state.get(&v) {
            Ok(v) => { *ast = val(v); true }
            Err(msg) => { *result = Err(msg); true }
        },
        AExpr::Add(ref mut a, ref mut b) => {
            match *(a.borrow_mut()) {
                AExpr::Value(a) => match *(b.borrow_mut()) {
                    AExpr::Value(b) => {
                        *ast = val(a + b);
                        true
                    }
                    _ => evalStep(b, state, result)
                },
                _ => evalStep(a, state, result)
            }
        },
        AExpr::Sub(ref mut a, ref mut b) => {
            match *(a.borrow_mut()) {
                AExpr::Value(a) => match *(b.borrow_mut()) {
                    AExpr::Value(b) => {
                        *ast = val(a - b);
                        true
                    }
                    _ => evalStep(b, state, result)
                },
                _ => evalStep(a, state, result)
            }
        },
        AExpr::Mul(ref mut a, ref mut b) => {
            match *(a.borrow_mut()) {
                AExpr::Value(a) => match *(b.borrow_mut()) {
                    AExpr::Value(b) => {
                        *ast = val(a * b);
                        true
                    }
                    _ => evalStep(b, state, result)
                },
                _ => evalStep(a, state, result)
            }
        },
    }
}
pub fn pureEvalStep (ast: &ARef, state: &State) -> Result<ARef, String> {
    return match *(ast.borrow()) {
        AExpr::Add(ref a, ref b) => pureEvalBinary(
            |a, b| a + b,
            |a, b| add(a, b),
            a, b, state),
        AExpr::Sub(ref a, ref b) => pureEvalBinary(
            |a, b| a - b,
            |a, b| sub(a, b),
            a, b, state),
        AExpr::Mul(ref a, ref b) => pureEvalBinary(
            |a, b| a * b,
            |a, b| mul(a, b),
            a, b, state),
        AExpr::Variable(ref v) => match state.get(&v) {
            Ok(a) => Ok(val(a)),
            Err(msg) => Err(msg),
        },
        AExpr::Value(_) => Ok(ast.clone())
    }
}
fn pureEvalBinary <F, C>(f: F, c: C, left: &ARef, right: &ARef, state: &State) -> Result<ARef, String>
    where F: FnOnce(Value, Value) -> Value, C: FnOnce(ARef, ARef) -> ARef
{
    return match *(left.borrow()) {
        AExpr::Value(a) => match *(right.borrow()) {
            AExpr::Value(b) => Ok(val(f(a, b))),
            _ => match pureEvalStep(right, state) {
                Ok(right) => Ok(c(left.clone(), right.clone())),
                Err(msg) => Err(msg),
            }
        },
        _ => match pureEvalStep(left, state) {
            Ok(left) => Ok(c(left.clone(), right.clone())),
            Err(msg) => Err(msg),
        }
    };
}






//macro_rules! implement_operator_ctor {
//    ( $Trait:ident :: $name:ident (
//        $CL:ident < $TL:ident > ,
//        $CR:ident < $TR:ident > ) ->
//        $Container:ident < $Type:ident :: $Tag:ident >
//    ) => {
//        impl $Trait for $CL<$TL> {
//            type Output = $Container<$Type>;
//            fn $name (self, rhs: $CR<$TR>) -> $Container<$Type> {
//                return $Container::new($Type::$Tag(self, rhs));
//            }
//        }
//    };
//    ( $Trait:ident :: $name:ident (
//        $CL:ident < $TL:ident > ) ->
//        $Container:ident < $Type:ident :: $Tag:ident >
//    ) => {
//        impl $Trait for $CL<$TL> {
//            type Output = $Container<$Type>;
//            fn $name (self) -> $Container<$Type> {
//                return $Container::new($Type::$Tag(self, rhs));
//            }
//        }
//    };
//}
//implement_operator_ctor!(Add::add (ARef, ARef) -> Rc<AExpr::Add>);
//implement_operator_ctor!(Sub::sub (ARef, ARef) -> Rc<AExpr::Sub>);
//implement_operator_ctor!(Mul::mul (ARef, ARef) -> Rc<AExpr::Mul>);
