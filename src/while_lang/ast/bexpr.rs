use crate::while_lang::types::Value;
use crate::while_lang::types::State;
use super::aexpr::AExpr;
use std::ops::Not;
use std::ops::BitOr;
use std::ops::BitAnd;

#[derive(Debug)]
pub enum BExpr {
    Value(bool),
    Not(Box<BExpr>),
    Or(Box<BExpr>, Box<BExpr>),
    And(Box<BExpr>, Box<BExpr>),
    Less(Box<AExpr>, Box<AExpr>),
    Equal(Box<AExpr>, Box<AExpr>),
}

pub fn eval (ast: Box<BExpr>, state: &State) -> Result<bool, String> {
    return match *ast {
        BExpr::Value(v) => Ok(v),
        BExpr::Not(a) => eval(a, state).map(|a| !a),
        BExpr::Or(a, b) => evalBinary(|a, b| a || b, a, b, state),
        BExpr::And(a, b) => evalBinary(|a, b| a && b, a, b, state),
        BExpr::Less(a, b) => evalCmp(|a, b| a < b, a, b, state),
        BExpr::Equal(a, b) => evalCmp(|a, b| a == b, a, b, state),
    }
}
fn evalBinary <F>(f: F, left: Box<BExpr>, right: Box<BExpr>, state: &State) -> Result<bool, String>
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
fn evalCmp <F>(f: F, left: Box<AExpr>, right: Box<AExpr>, state: &State) -> Result<bool, String>
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

type BoxedAExpr = Box<AExpr>;
type BoxedBExpr = Box<BExpr>;

pub fn btrue () -> BoxedBExpr { return Box::new(BExpr::Value(true)); }
pub fn bfalse () -> BoxedBExpr { return Box::new(BExpr::Value(false)); }
make_ctor!(not (expr: BoxedBExpr) -> Box<BExpr::Not>);
make_ctor!(or (left: BoxedBExpr, right: BoxedBExpr) -> Box<BExpr::Or>);
make_ctor!(and (left: BoxedBExpr, right: BoxedBExpr) -> Box<BExpr::And>);
make_ctor!(equal (left: BoxedAExpr, right: BoxedAExpr) -> Box<BExpr::Equal>);
make_ctor!(less (left: BoxedAExpr, right: BoxedAExpr) -> Box<BExpr::Less>);

macro_rules!implement_operator_ctor {
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
                return $Container::new($Type::$Tag(self));
            }
        }
    };
}
implement_operator_ctor!(Not::not (Box<BExpr>) -> Box<BExpr::Not>);
implement_operator_ctor!(BitAnd::bitand (Box<BExpr>, Box<BExpr>) -> Box<BExpr::And>);
implement_operator_ctor!(BitOr::bitor (Box<BExpr>, Box<BExpr>) -> Box<BExpr::Or>);

// no Equal or Less overloads b/c rust does not support arbitrary operator overloading
// who on earth would possibly need to use that...? -_-
