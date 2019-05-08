use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use crate::while_lang::types::State;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub enum AExpr {
    Value(Value),
    Variable(Variable),
    Add(Box<AExpr>, Box<AExpr>),
    Sub(Box<AExpr>, Box<AExpr>),
    Mul(Box<AExpr>, Box<AExpr>),
}
pub fn pureEvalStep (ast: Box<AExpr>, state: &State) -> Result<Box<AExpr>, String> {
    return match *ast {
        AExpr::Add(a, b) => pureEvalBinary(
            |a, b| a + b,
            |a, b| Box::new(AExpr::Add(a, b)),
            a, b, state),
        AExpr::Sub(a, b) => pureEvalBinary(
            |a, b| a - b,
            |a, b| Box::new(AExpr::Sub(a, b)),
            a, b, state),
        AExpr::Mul(a, b) => pureEvalBinary(
            |a, b| a * b,
            |a, b| Box::new(AExpr::Mul(a, b)),
            a, b, state),
        AExpr::Variable(v) => match state.get(&v) {
            Ok(a) => Box::new(AExpr::Value),
            Err(msg) => Err(msg),
        },
        AExpr::Value(_) => Ok(ast)
    }
}
fn pureEvalBinary <F, C>(f: F, c: C, left: Box<AExpr>, right: Box<AExpr>, state: &State) -> Result<Box<AExpr>, String>
    where F: FnOnce(Value, Value) -> Value, C: FnOnce(Box<AExpr>, Box<AExpr>) -> Box<AExpr>
{
    return match left {
        AExpr::Value(a) => match right {
            AExpr::Value(b) => Ok(Box::new(AExpr::Value(f(a, b)))),
            _ => match pureEvalStep(right, state) {
                Ok(right) => Ok(c(left, right)),
                Err(msg) => Err(msg),
            }
        },
        _ => match pureEvalStep(left, state) {
            Ok(left) => Ok(c(left, right)),
            Err(msg) => Err(msg),
        }
    };
}



fn unsafeGetValue (ast: &Box<AExpr>) -> Value {
    return match **ast {
        AExpr::Value(a) => a,
        _ => panic!(format!("{:?} is not a value!", *ast))
    }
}
pub fn evalStep (ast: &mut Box<AExpr>, state: &State, result: &mut Result<bool, String>) -> bool {
    return match **ast {
        AExpr::Add(ref mut a, ref mut b) => {
            if !evalStep(a, state, result) && !evalStep(b, state, result) {
                *ast = Box::new(AExpr::Value(unsafeGetValue(&a) + unsafeGetValue(&b)));
            }
            true
        },
        AExpr::Sub(ref mut a, ref mut b) => {
            if !evalStep(a, state, result) && !evalStep(b, state, result) {
                *ast = Box::new(AExpr::Value(unsafeGetValue(&a) - unsafeGetValue(&b)));
            }
            true
        },
        AExpr::Mul(ref mut a, ref mut b) => {
            if !evalStep(a, state, result) && !evalStep(b, state, result) {
                *ast = Box::new(AExpr::Value(unsafeGetValue(&a) * unsafeGetValue(&b)));
            }
            true
        },
        AExpr::Variable(ref v) => {
            match state.get(&v) {
                Ok(a) => *ast = Box::new(AExpr::Value(a)),
                Err(msg) => *result = Err(msg),
            }
            true
        },
        AExpr::Value(_) => false
    }
}
pub fn eval (ast: Box<AExpr>, state: &State) -> Result<Value, String> {
    return match *ast {
        AExpr::Value(v) => Ok(v),
        AExpr::Variable(v) => state.get(&v),
        AExpr::Add(a, b) => evalBinary(|a, b| a + b, a, b, state),
        AExpr::Sub(a, b) => evalBinary(|a, b| a - b, a, b, state),
        AExpr::Mul(a, b) => evalBinary(|a, b| a * b, a, b, state),
    }
}
fn evalBinary <F>(f: F, left: Box<AExpr>, right: Box<AExpr>, state: &State) -> Result<Value, String>
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

macro_rules! make_ctor {
    ( $name:ident ($x1:ident: $t1:ident) -> $Container:ident < $Type:ident :: $Tag:ident > ) => {
        pub fn $name ($x1: $t1) -> $Container<$Type> {
            return $Container::new($Type::$Tag($x1));
        }
    };
    ( $name:ident ($x1:ident: $t1:ident, $x2:ident: $t2:ident) -> $Container:ident < $Type:ident :: $Tag:ident > ) => {
        pub fn $name ($x1: $t1, $x2: $t2) -> $Container<$Type> {
            return $Container::new($Type::$Tag($x1, $x2));
        }
    };
}
type BoxedAExpr = Box<AExpr>;
make_ctor!(val (v: Value) -> Box<AExpr::Value>);
pub fn var (v: &str) -> BoxedAExpr { return Box::new(AExpr::Variable(v.to_string())); }
make_ctor!(add (left: BoxedAExpr, right: BoxedAExpr) -> Box<AExpr::Add>);
make_ctor!(sub (left: BoxedAExpr, right: BoxedAExpr) -> Box<AExpr::Sub>);
make_ctor!(mul (left: BoxedAExpr, right: BoxedAExpr) -> Box<AExpr::Mul>);

macro_rules! implement_operator_ctor {
    ( $Trait:ident :: $name:ident (
        $CL:ident < $TL:ident > ,
        $CR:ident < $TR:ident > ) ->
        $Container:ident < $Type:ident :: $Tag:ident >
    ) => {
        impl $Trait for $CL<$TL> {
            type Output = $Container<$Type>;
            fn $name (self, rhs: $CR<$TR>) -> $Container<$Type> {
                return $Container::new($Type::$Tag(self, rhs));
            }
        }
    };
    ( $Trait:ident :: $name:ident (
        $CL:ident < $TL:ident > ) ->
        $Container:ident < $Type:ident :: $Tag:ident >
    ) => {
        impl $Trait for $CL<$TL> {
            type Output = $Container<$Type>;
            fn $name (self) -> $Container<$Type> {
                return $Container::new($Type::$Tag(self, rhs));
            }
        }
    };
}
implement_operator_ctor!(Add::add (Box<AExpr>, Box<AExpr>) -> Box<AExpr::Add>);
implement_operator_ctor!(Sub::sub (Box<AExpr>, Box<AExpr>) -> Box<AExpr::Sub>);
implement_operator_ctor!(Mul::mul (Box<AExpr>, Box<AExpr>) -> Box<AExpr::Mul>);
