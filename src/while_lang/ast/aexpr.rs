use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[derive(Debug)]
pub enum AExpr {
    Value(Value),
    Variable(Variable),
    Add(Box<AExpr>, Box<AExpr>),
    Sub(Box<AExpr>, Box<AExpr>),
    Mul(Box<AExpr>, Box<AExpr>),
}
type BoxedAExpr = Box<AExpr>;

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
make_ctor!(val (v: Value) -> Box<AExpr::Value>);
make_ctor!(var (v: Variable) -> Box<AExpr::Variable>);
make_ctor!(add (left: BoxedAExpr, right: BoxedAExpr) -> Box<AExpr::Add>);
make_ctor!(sub (left: BoxedAExpr, right: BoxedAExpr) -> Box<AExpr::Sub>);
make_ctor!(mul (left: BoxedAExpr, right: BoxedAExpr) -> Box<AExpr::Mul>);

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
                return $Container::new($Type::$Tag(self, rhs));
            }
        }
    };
}

implement_operator_ctor!(Add::add (Box<AExpr>, Box<AExpr>) -> Box<AExpr::Add>);
implement_operator_ctor!(Sub::sub (Box<AExpr>, Box<AExpr>) -> Box<AExpr::Sub>);
implement_operator_ctor!(Mul::mul (Box<AExpr>, Box<AExpr>) -> Box<AExpr::Mul>);
