use crate::while_lang::types::Value;
use crate::while_lang::types::State;
use super::aexpr::AExpr;
//use std::ops::Not;
//use std::ops::BitOr;
//use std::ops::BitAnd;
use std::rc::Rc;
use super::aexpr;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BExpr {
    Value(bool),
    Not(Rc<BExpr>),
    Or(Rc<BExpr>, Rc<BExpr>),
    And(Rc<BExpr>, Rc<BExpr>),
    Less(Box<AExpr>, Box<AExpr>),
    Equal(Box<AExpr>, Box<AExpr>),
}
pub fn evalStep (ast: &mut Rc<BExpr>, state: &State, result: &mut Result<bool, String>) -> bool {
    return match **ast {
        BExpr::Value(v) => false,
        BExpr::Not(ref mut a) => match **a {
            BExpr::Value(true) => { *ast = bfalse.clone(); true }
            BExpr::Value(false) => { *ast = btrue.clone(); true }
            _ => evalStep(a, state, result)
        },
        BExpr::Or(ref mut a, ref mut b) => match **a {
            BExpr::Value(true) => { *ast = a.clone(); true },
            BExpr::Value(false) => match **b {
                BExpr::Value(_) => { *ast = b.clone(); true },
                _ => evalStep(b, state, result)
            },
            _ => evalStep(a, state, result)
        },
        BExpr::And(ref mut a, ref mut b) => match **a {
            BExpr::Value(false) => { *ast = a.clone(); true },
            BExpr::Value(true) => match **b {
                BExpr::Value(_) => { *ast = b.clone(); true },
                _ => evalStep(b, state, result)
            },
            _ => evalStep(a, state, result)
        },
        BExpr::Less(ref mut a, ref mut b) => match **a {
            AExpr::Value(ref mut a) => match **b {
                AExpr::Value(ref mut b) => match a < b {
                    true => { *ast = btrue.clone(); true },
                    false => { *ast = bfalse.clone(); true },
                },
                _ => aexpr::evalStep(b, state, result)
            },
            _ => aexpr::evalStep(a, state, result)
        },
        BExpr::Equal(ref mut a, ref mut b) => match **a {
            AExpr::Value(ref mut a) => match **b {
                AExpr::Value(ref mut b) => match a == b {
                    true => { *ast = btrue.clone(); true },
                    false => { *ast = bfalse.clone(); true },
                },
                _ => aexpr::evalStep(b, state, result)
            },
            _ => aexpr::evalStep(a, state, result)
        },
    }
}

pub fn eval (ast: Rc<BExpr>, state: &State) -> Result<bool, String> {
    return match *ast {
        BExpr::Value(ref v) => Ok(*v),
        BExpr::Not(ref a) => eval(*a, state).map(|a| !a),
        BExpr::Or(ref a, ref b) => evalBinary(|a, b| a || b, a, b, state),
        BExpr::And(ref a, ref b) => evalBinary(|a, b| a && b, a, b, state),
        BExpr::Less(ref a, ref b) => evalCmp(|a, b| a < b, a, b, state),
        BExpr::Equal(ref a, ref b) => evalCmp(|a, b| a == b, a, b, state),
    }
}
fn evalBinary <F>(f: F, left: Rc<BExpr>, right: Rc<BExpr>, state: &State) -> Result<bool, String>
    where F: FnOnce(bool, bool) -> bool
{
    return match eval(left, state) {
        Ok(a) => match eval(right, state) {
            Ok(b) => Ok(f(a, b)),
            err => err,
        },
        err => err
    };
}
fn evalCmp <F>(f: F, left: Rc<AExpr>, right: Rc<AExpr>, state: &State) -> Result<bool, String>
    where F: FnOnce(Value, Value) -> bool
{
    use super::aexpr;
    return match aexpr::eval(left, state) {
        Ok(a) => match aexpr::eval(right, state) {
            Ok(b) => Ok(f(a, b)),
            Err(msg) => Err(msg),
        },
        Err(msg) => Err(msg)
    };
}

macro_rules!make_ctor {
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
type RcedBExpr = Rc<BExpr>;

pub fn btrue () -> RcedBExpr { return Rc::new(BExpr::Value(true)); }
pub fn bfalse () -> RcedBExpr { return Rc::new(BExpr::Value(false)); }
make_ctor!(not (expr: RcedBExpr) -> Rc<BExpr::Not>);
make_ctor!(or (left: RcedBExpr, right: RcedBExpr) -> Rc<BExpr::Or>);
make_ctor!(and (left: RcedBExpr, right: RcedBExpr) -> Rc<BExpr::And>);
make_ctor!(equal (left: RcedAExpr, right: RcedAExpr) -> Rc<BExpr::Equal>);
make_ctor!(less (left: RcedAExpr, right: RcedAExpr) -> Rc<BExpr::Less>);

//macro_rules!implement_operator_ctor {
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
//                return $Container::new($Type::$Tag(self));
//            }
//        }
//    };
//}
//implement_operator_ctor!(Not::not (Rc<BExpr>) -> Rc<BExpr::Not>);
//implement_operator_ctor!(BitAnd::bitand (Rc<BExpr>, Rc<BExpr>) -> Rc<BExpr::And>);
//implement_operator_ctor!(BitOr::bitor (Rc<BExpr>, Rc<BExpr>) -> Rc<BExpr::Or>);

// no Equal or Less overloads b/c rust does not support arbitrary operator overloading
// who on earth would possibly need to use that...? -_-
