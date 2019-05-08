mod while_lang;

#[cfg(test)]
mod tests {
    #[test]
    fn arith () {
        use crate::while_lang::ast::AExpr;
        use crate::while_lang::ast::value;
        use crate::while_lang::eval;
        assert_eq!(eval(value(10)), 10);
        assert_eq!(eval(value(-10)), -10);
        assert_eq!(eval(value(2) + value(4)), 6);
        assert_eq!(eval(value(2) - value(4)), -2);
    }
}
