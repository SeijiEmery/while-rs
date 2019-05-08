mod while_lang;

#[cfg(test)]
mod tests {
    #[test]
    fn arith () {
        use crate::while_lang::ast::aexpr;
        assert_eq!(eval(AExpr::Value(10)), 10);
        assert_eq!(eval(Value(-10)), -10);
    }
}
