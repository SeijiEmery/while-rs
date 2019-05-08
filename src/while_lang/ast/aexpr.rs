
#[derive(Debug)]
pub enum AExpr <'a> {
    Value(i64),
    Add(&'a AExpr<'a>, &'a AExpr<'a>),
    Sub(&'a AExpr<'a>, &'a AExpr<'a>),
}
//pub fn value <'a> (v: i64) -> &'a AExpr<'a> {
//    let result = AExpr::Value(v);
//    return &result;
//}
pub fn eval <'a> (expr: &'a AExpr<'a>) -> i64 {
    match expr {
        AExpr::Value(v) => *v,
        AExpr::Add(a, b) => eval(a) + eval(b),
        AExpr::Sub(a, b) => eval(a) - eval(b),
    }
}

#[test]
pub fn example () {
    let a = AExpr::Value(10);
    let b = AExpr::Value(12);
    let c = AExpr::Value(4);
    let s = AExpr::Sub(&a, &b);
    let expr = AExpr::Add(&c, &s);
    assert_eq!(eval(&expr), -18);
}










//pub fn value <'a> (v: i64) -> &'a mut AExpr<'a>

//    return AExpr::Value(v);
//}
//use std::ops;
//impl <'a> ops::Add<AExpr<'a>> for AExpr<'a>
//    type Output = AExpr<'a>;
//    fn add(self: &mut AExpr<'a>, rhs: &mut AExpr<'a>) -> AExpr<'a> {
//        return AExpr::Add(self, rhs);
//    }
//}

//im
// pl ops::Sub<AExp> for AExp {
//    type Output = AExp;
//    fn sub (self, rhs: AExp) -> AExp {
//
//
//
//
//
//       return Box::new(AExpr::Sub(self, rhs));
//    }
//}
//fn foo () -> AExp {
//    return value(10) + value(20);
//}
