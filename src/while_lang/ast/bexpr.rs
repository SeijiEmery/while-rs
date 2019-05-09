use crate::while_lang::types::Value;
use crate::while_lang::types::State;
use super::aexpr::AExpr;
use super::aexpr::ARef;
//use std::ops::Not;
//use std::ops::BitOr;
//use std::ops::BitAnd;
use std::rc::Rc;
use std::cell::RefCell;
use super::aexpr;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum BExpr {
    Value(bool),
    Not(BRef),
    Or(BRef, BRef),
    And(BRef, BRef),
    Less(ARef, ARef),
    Equal(ARef, ARef),
}
pub type BRef = Rc<RefCell<BExpr>>;

// constants (reference these, so sort of only defined once...)
static TRUE : BRef = Rc::new(RefCell::new(BExpr::Value(true)));
static FALSE : BRef = Rc::new(RefCell::new(BExpr::Value(false)));

// constructors
pub fn btrue () -> BRef { TRUE.clone() }
pub fn bfalse () -> BRef { FALSE.clone() }
pub fn not (a: BRef) -> BRef { Rc::new(RefCell::new(BExpr::Not(a))) }
pub fn or (a: BRef, b: BRef) -> BRef { Rc::new(RefCell::new(BExpr::Or(a, b))) }
pub fn and (a: BRef, b: BRef) -> BRef { Rc::new(RefCell::new(BExpr::And(a, b))) }
pub fn less (a: ARef, b: ARef) -> BRef { Rc::new(RefCell::new(BExpr::Less(a, b))) }
pub fn equal (a: ARef, b: ARef) -> BRef { Rc::new(RefCell::new(BExpr::Equal(a, b))) }

// eager eval
pub fn eval (ast: &BRef, state: &State) -> Result<bool, String> {
    return match *(ast.borrow()) {
        BExpr::Value(v) => Ok(v),
        BExpr::Not(ref a) => eval(a, state).map(|a| !a),
        BExpr::Or(ref a, ref b) => evalBinary(|a, b| a || b, a, b, state),
        BExpr::And(ref a, ref b) => evalBinary(|a, b| a && b, a, b, state),
        BExpr::Less(ref a, ref b) => evalCmp(|a, b| a < b, a, b, state),
        BExpr::Equal(ref a, ref b) => evalCmp(|a, b| a == b, a, b, state),
    }
}
fn evalBinary <F>(f: F, left: &BRef, right: &BRef, state: &State) -> Result<bool, String>
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
fn evalCmp <F>(f: F, left: &ARef, right: &ARef, state: &State) -> Result<bool, String>
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

// lazy single-step eval
pub fn evalStep (ast: &mut BRef, state: &State, result: &mut Result<bool, String>) -> bool {
    return match *(ast.borrow_mut()) {
        BExpr::Value(_) => false,
        BExpr::Not(ref mut a) => match *(a.borrow_mut()) {
            BExpr::Value(true) => { *ast = bfalse().clone(); true }
            BExpr::Value(false) => { *ast = btrue().clone(); true }
            _ => evalStep(a, state, result)
        },
        BExpr::Or(ref mut a, ref mut b) => match *(a.borrow_mut()) {
            BExpr::Value(true) => { *ast = a.clone(); true },
            BExpr::Value(false) => match *(b.borrow_mut()) {
                BExpr::Value(_) => { *ast = b.clone(); true },
                _ => evalStep(b, state, result)
            },
            _ => evalStep(a, state, result)
        },
        BExpr::And(ref mut a, ref mut b) => match *(a.borrow_mut()) {
            BExpr::Value(false) => { *ast = a.clone(); true },
            BExpr::Value(true) => match *(b.borrow_mut()) {
                BExpr::Value(_) => { *ast = b.clone(); true },
                _ => evalStep(b, state, result)
            },
            _ => evalStep(a, state, result)
        },
        BExpr::Less(ref mut a, ref mut b) => match *(a.borrow_mut()) {
            AExpr::Value(ref mut a) => match *(b.borrow_mut()) {
                AExpr::Value(ref mut b) => match a < b {
                    true => { *ast = btrue(); true },
                    false => { *ast = bfalse(); true },
                },
                _ => aexpr::evalStep(b, state, result)
            },
            _ => aexpr::evalStep(a, state, result)
        },
        BExpr::Equal(ref mut a, ref mut b) => match *(a.borrow_mut()) {
            AExpr::Value(ref mut a) => match *(b.borrow_mut()) {
                AExpr::Value(ref mut b) => match a == b {
                    true => { *ast = btrue(); true },
                    false => { *ast = bfalse(); true },
                },
                _ => aexpr::evalStep(b, state, result)
            },
            _ => aexpr::evalStep(a, state, result)
        },
    }
}



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
//implement_operator_ctor!(Not::not (BRef) -> Rc<BExpr::Not>);
//implement_operator_ctor!(BitAnd::bitand (BRef, BRef) -> Rc<BExpr::And>);
//implement_operator_ctor!(BitOr::bitor (BRef, BRef) -> Rc<BExpr::Or>);

// no Equal or Less overloads b/c rust does not support arbitrary operator overloading
// who on earth would possibly need to use that...? -_-
