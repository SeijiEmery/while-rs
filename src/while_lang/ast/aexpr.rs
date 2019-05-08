use crate::while_lang::types::Value;
use crate::while_lang::types::Variable;

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
