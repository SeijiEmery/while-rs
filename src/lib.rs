#![feature(concat_idents)]
mod while_lang;

macro_rules!impl_ast {
    ($name:ident val $x: expr) => {
        let $name = AExpr::Value($x);
    };
    ($name:ident add ( $($left:tt)* ) ( $($right:tt)* )) => {
        impl_ast!(concat_idents!($name, l) $($left)*);
        impl_ast!(concat_idents!($name, r) $($right)*);
        let $name = AExpr::Add(&concat_idents!($name, l), &concat_idents!($name, r));
    }
}
#[cfg(test)]
mod tests {
    use crate::while_lang::ast::aexpr::AExpr;

    #[test]
    fn arith () {
//        let ast = add!(val!(10), val!(-2));

        impl_ast!( x val 10 );
        impl_ast!( y add (val 10) (val 2));

//        impl_ast!( y add (val 10) (sub (val 20) (val 30)));


        use crate::while_lang::ast::AExpr;
//        use crate::while_lang::ast::value;
        use crate::while_lang::eval;

        let a = AExpr::Value(10);
        let b = AExpr::Value(12);
        let c = AExpr::Value(4);
        let s = AExpr::Sub(&a, &b);
        let expr = AExpr::Add(&c, &s);
        assert_eq!(2, eval(&expr));

//        assert_eq!(eval(&mut AExpr::Value(10)), 10);

//        assert_eq!(eval(value(10)), 10);
//        assert_eq!(eval(value(-10)), -10);
//        assert_eq!(eval(value(2) + value(4)), 6);
//        assert_eq!(eval(value(2) - value(4)), -2);
    }
}
