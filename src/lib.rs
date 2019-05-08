#![macro_use]
mod while_lang;

#[cfg(test)]
mod tests {
    #[test]
    fn arith_tests () {
        use crate::while_lang::ast::aexpr::val;
        use crate::while_lang::ast::aexpr::var;
        use crate::while_lang::ast::aexpr::eval;
        use crate::while_lang::HashState;

        let empty = HashState::new();
        let mut x10 = HashState::new();
        x10.insert("x".to_string(), 10);

        assert_eq!(eval(val(10), &empty), Ok(10));
        assert_eq!(eval(val(-10), &empty), Ok(-10));
        assert_eq!(eval(var("x"), &empty).is_err(), true);
        assert_eq!(eval(var("x"), &x10), Ok(10));
        assert_eq!(eval(val(2) + val(4), &empty), Ok(6));
        assert_eq!(eval(val(2) - val(4), &empty), Ok(-2));
        assert_eq!(eval(val(2) * val(4), &empty), Ok(8));
    }

    #[test]
    fn bool_tests () {
        use crate::while_lang::ast::bexpr::btrue;
        use crate::while_lang::ast::bexpr::bfalse;
        use crate::while_lang::ast::bexpr::eval;
        use crate::while_lang::ast::bexpr::equal;
        use crate::while_lang::ast::bexpr::less;
        use crate::while_lang::ast::bexpr::not;
        use crate::while_lang::ast::bexpr::and;
        use crate::while_lang::ast::bexpr::or;
        use crate::while_lang::ast::aexpr::val;
        use crate::while_lang::ast::aexpr::var;
        use crate::while_lang::HashState;

        let empty = HashState::new();
        let mut x10 = HashState::new();
        x10.insert("x".to_string(), 10);

        assert_eq!(eval(btrue(), &empty), Ok(true));
        assert_eq!(eval(bfalse(), &empty), Ok(false));
        assert_eq!(eval(not(btrue()), &empty), Ok(false));
        assert_eq!(eval(not(bfalse()), &empty), Ok(true));

        assert_eq!(eval(or(btrue(), btrue()), &empty), Ok(true));
        assert_eq!(eval(or(btrue(), bfalse()), &empty), Ok(true));
        assert_eq!(eval(or(bfalse(), btrue()), &empty), Ok(true));
        assert_eq!(eval(or(bfalse(), bfalse()), &empty), Ok(false));

        assert_eq!(eval(and(btrue(), btrue()), &empty), Ok(true));
        assert_eq!(eval(and(btrue(), bfalse()), &empty), Ok(false));
        assert_eq!(eval(and(bfalse(), btrue()), &empty), Ok(false));
        assert_eq!(eval(and(bfalse(), bfalse()), &empty), Ok(false));

        assert_eq!(eval(equal(val(10), var("x")), &x10), Ok(true));
        assert_eq!(eval(equal(var("x"), val(10)), &x10), Ok(true));
        assert_eq!(eval(equal(val(9), var("x")), &x10), Ok(false));
        assert_eq!(eval(equal(val(11), var("x")), &x10), Ok(false));
        assert_eq!(eval(equal(val(10), var("y")), &x10).is_err(), true);

        assert_eq!(eval(less(val(10), var("x")), &x10), Ok(false));
        assert_eq!(eval(less(val(9), var("x")), &x10), Ok(true));
        assert_eq!(eval(less(var("x"), val(11)), &x10), Ok(true));
        assert_eq!(eval(less(val(11), var("x")), &x10), Ok(false));
        assert_eq!(eval(less(val(10), var("y")), &x10).is_err(), true);
    }
}
