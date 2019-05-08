mod while_lang;

#[cfg(test)]
mod tests {
    use crate::while_lang::ast::aexpr::AExpr;

    #[test]
    fn arith () {
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
