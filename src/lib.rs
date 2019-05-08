mod while_lang;

#[cfg(test)]
mod tests {
    #[test]
    fn arith_tests () {
        use crate::while_lang::ast::val;
        use crate::while_lang::ast::var;
        use crate::while_lang::ast::add;
        use crate::while_lang::ast::sub;
        use crate::while_lang::ast::mul;
        use crate::while_lang::eval;
        use crate::while_lang::HashState;

        let empty = HashState::new();
        let mut x10 = HashState::new();
        x10.insert("x".to_string(), 10);

        assert_eq!(eval(val(10), &empty), Ok(10));
        assert_eq!(eval(val(-10), &empty), Ok(-10));
        assert_eq!(eval(var("x".to_string()), &empty).is_err(), true);
        assert_eq!(eval(var("x".to_string()), &x10), Ok(10));
        assert_eq!(eval(add(val(2), val(4)), &empty), Ok(6));
        assert_eq!(eval(sub(val(2), val(4)), &empty), Ok(-2));
        assert_eq!(eval(mul(val(2), val(4)), &empty), Ok(8));
    }

    #[test]
    fn bool_tests () {}
}
