use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use crate::while_lang::types::State;
//use std::ops::Add;
//use std::ops::Sub;
//use std::ops::Mul;
use std::rc::Rc;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum AExpr {
    Value(Value),
    Variable(Variable),
    Add(Rc<AExpr>, Rc<AExpr>),
    Sub(Rc<AExpr>, Rc<AExpr>),
    Mul(Rc<AExpr>, Rc<AExpr>),
}
pub fn pureEvalStep (ast: &Rc<AExpr>, state: &State) -> Result<Rc<AExpr>, String> {
    return match **ast {
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
fn pureEvalBinary <F, C>(f: F, c: C, left: &Rc<AExpr>, right: &Rc<AExpr>, state: &State) -> Result<Rc<AExpr>, String>
    where F: FnOnce(Value, Value) -> Value, C: FnOnce(Rc<AExpr>, Rc<AExpr>) -> Rc<AExpr>
{
    return match **left {
        AExpr::Value(a) => match **right {
            AExpr::Value(b) => Ok(Rc::new(AExpr::Value(f(a, b)))),
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



fn unsafeGetValue (ast: &Rc<AExpr>) -> Value {
    return match **ast {
        AExpr::Value(a) => a,
        _ => panic!(format!("{:?} is not a value!", *ast))
    }
}
pub fn evalStep (ast: &mut Rc<AExpr>, state: &State, result: &mut Result<bool, String>) -> bool {
    return match **ast {
        AExpr::Add(ref mut a, ref mut b) => {
            if !evalStep(a, state, result) && !evalStep(b, state, result) {
                *ast = Rc::new(AExpr::Value(unsafeGetValue(&a) + unsafeGetValue(&b)));
            }
            true
        },
        AExpr::Sub(ref mut a, ref mut b) => {
            if !evalStep(a, state, result) && !evalStep(b, state, result) {
                *ast = Rc::new(AExpr::Value(unsafeGetValue(&a) - unsafeGetValue(&b)));
            }
            true
        },
        AExpr::Mul(ref mut a, ref mut b) => {
            if !evalStep(a, state, result) && !evalStep(b, state, result) {
                *ast = Rc::new(AExpr::Value(unsafeGetValue(&a) * unsafeGetValue(&b)));
            }
            true
        },
        AExpr::Variable(ref v) => {
            match state.get(&v) {
                Ok(a) => *ast = Rc::new(AExpr::Value(a)),
                Err(msg) => *result = Err(msg),
            }
            true
        },
        AExpr::Value(_) => false
    }
}
pub fn eval (ast: &Rc<AExpr>, state: &State) -> Result<Value, String> {
    return match **ast {
        AExpr::Value(v) => Ok(v),
        AExpr::Variable(ref v) => state.get(&v),
        AExpr::Add(ref a, ref b) => evalBinary(|a, b| a + b, a, b, state),
        AExpr::Sub(ref a, ref b) => evalBinary(|a, b| a - b, a, b, state),
        AExpr::Mul(ref a, ref b) => evalBinary(|a, b| a * b, a, b, state),
    }
}
fn evalBinary <F>(f: F, left: &Rc<AExpr>, right: &Rc<AExpr>, state: &State) -> Result<Value, String>
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
type RcedAExpr = Rc<AExpr>;
make_ctor!(val (v: Value) -> Rc<AExpr::Value>);
pub fn var (v: &str) -> RcedAExpr { return Rc::new(AExpr::Variable(v.to_string())); }
make_ctor!(add (left: RcedAExpr, right: RcedAExpr) -> Rc<AExpr::Add>);
make_ctor!(sub (left: RcedAExpr, right: RcedAExpr) -> Rc<AExpr::Sub>);
make_ctor!(mul (left: RcedAExpr, right: RcedAExpr) -> Rc<AExpr::Mul>);

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
//implement_operator_ctor!(Add::add (Rc<AExpr>, Rc<AExpr>) -> Rc<AExpr::Add>);
//implement_operator_ctor!(Sub::sub (Rc<AExpr>, Rc<AExpr>) -> Rc<AExpr::Sub>);
//implement_operator_ctor!(Mul::mul (Rc<AExpr>, Rc<AExpr>) -> Rc<AExpr::Mul>);
