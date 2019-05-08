mod while_lang;

#[cfg(test)]
mod tests {
    use crate::while_lang::ast::aexpr::AExpr;

    #[test]
    fn arith () {
        use crate::while_lang::ast::AExpr;
        use crate::while_lang::ast::val;
        use crate::while_lang::ast::var;
        use crate::while_lang::ast::add;
        use crate::while_lang::ast::sub;
        use crate::while_lang::ast::mul;
        use crate::while_lang::eval;

        assert_eq!(eval(val(10)), 10);
        assert_eq!(eval(val(-10)), -10);
        assert_eq!(eval(add(val(2), val(4))), 6);
        assert_eq!(eval(sub(val(2), val(4))), -2);
    }
}
