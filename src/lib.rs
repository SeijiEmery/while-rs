#![macro_use]
mod while_lang;

#[cfg(test)]
mod tests {
//    #[test]
//    fn arith_tests () {
//        use crate::while_lang::ast::aexpr::val;
//        use crate::while_lang::ast::aexpr::var;
//        use crate::while_lang::ast::aexpr::add;
//        use crate::while_lang::ast::aexpr::sub;
//        use crate::while_lang::ast::aexpr::mul;
//        use crate::while_lang::ast::aexpr::eval;
//        use crate::while_lang::HashState;
//
//        let empty = HashState::new();
//        let mut x10 = HashState::new();
//        x10.insert("x".to_string(), 10);
//
//        assert_eq!(eval(&val(10), &empty), Ok(10));
//        assert_eq!(eval(&val(-10), &empty), Ok(-10));
//        assert_eq!(eval(&var("x"), &empty).is_err(), true);
//        assert_eq!(eval(&var("x"), &x10), Ok(10));
//        assert_eq!(eval(&add(val(2), val(4)), &empty), Ok(6));
//        assert_eq!(eval(&sub(val(2), val(4)), &empty), Ok(-2));
//        assert_eq!(eval(&mul(val(2), val(4)), &empty), Ok(8));
//    }
/*
    #[test]
    fn arith_step_tests () {
        use crate::while_lang::ast::aexpr::val;
        use crate::while_lang::ast::aexpr::var;
        use crate::while_lang::ast::aexpr::add;
        use crate::while_lang::ast::aexpr::sub;
        use crate::while_lang::ast::aexpr::mul;
        use crate::while_lang::ast::aexpr::evalStep;
        use crate::while_lang::HashState;

        let empty = HashState::new();
        let mut x10 = HashState::new();
        x10.insert("x".to_string(), 10);

        {
            let mut ast = val(10);
            let state = &empty;
            let mut result = Ok(true);
            assert_eq!(evalStep(&mut ast, state, &mut result), false);
            assert_eq!(ast, val(10));
            assert_eq!(*state, empty);
            assert_eq!(result, Ok(true));
        }
        {
            let mut ast = var("x");
            let state = &empty;
            let mut result = Ok(true);
            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), true);
            assert_eq!(*state, empty);
        }
        {
            let mut ast = var("x");
            let state = &x10;
            let mut result = Ok(true);
            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), false);
            assert_eq!(*ast, *val(10));

            assert_eq!(evalStep(&mut ast, state, &mut result), false);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, val(10));
        }
        {
            let mut ast = add(var("x"), val(10));
            let state = &x10;
            let mut result = Ok(true);
            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, add(val(10), val(10)));

            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, val(20));

            assert_eq!(evalStep(&mut ast, state, &mut result), false);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, val(20));
        }
        {
            let mut ast = add(var("x"), var("y"));
            let state = &x10;
            let mut result = Ok(true);
            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, add(val(10), var("y")));

            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), true);
        }
        {
            let mut ast = sub(mul(var("x"), val(10)), val(20));
            let state = &x10;
            let mut result = Ok(true);
            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, sub(mul(val(10), val(10)), val(20)));

            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, sub(val(20), val(20)));

            assert_eq!(evalStep(&mut ast, state, &mut result), true);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, val(0));

            assert_eq!(evalStep(&mut ast, state, &mut result), false);
            assert_eq!(result.is_err(), false);
            assert_eq!(ast, val(0));
        }
    }
*//*
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

        assert_eq!(eval(&btrue(), &empty), Ok(true));
        assert_eq!(eval(&bfalse(), &empty), Ok(false));
        assert_eq!(eval(&not(btrue()), &empty), Ok(false));
        assert_eq!(eval(&not(bfalse()), &empty), Ok(true));

        assert_eq!(eval(&or(btrue(), btrue()), &empty), Ok(true));
        assert_eq!(eval(&or(btrue(), bfalse()), &empty), Ok(true));
        assert_eq!(eval(&or(bfalse(), btrue()), &empty), Ok(true));
        assert_eq!(eval(&or(bfalse(), bfalse()), &empty), Ok(false));

        assert_eq!(eval(&and(btrue(), btrue()), &empty), Ok(true));
        assert_eq!(eval(&and(btrue(), bfalse()), &empty), Ok(false));
        assert_eq!(eval(&and(bfalse(), btrue()), &empty), Ok(false));
        assert_eq!(eval(&and(bfalse(), bfalse()), &empty), Ok(false));

        assert_eq!(eval(&equal(val(10), var("x")), &x10), Ok(true));
        assert_eq!(eval(&equal(var("x"), val(10)), &x10), Ok(true));
        assert_eq!(eval(&equal(val(9), var("x")), &x10), Ok(false));
        assert_eq!(eval(&equal(val(11), var("x")), &x10), Ok(false));
        assert_eq!(eval(&equal(val(10), var("y")), &x10).is_err(), true);

        assert_eq!(eval(&less(val(10), var("x")), &x10), Ok(false));
        assert_eq!(eval(&less(val(9), var("x")), &x10), Ok(true));
        assert_eq!(eval(&less(var("x"), val(11)), &x10), Ok(true));
        assert_eq!(eval(&less(val(11), var("x")), &x10), Ok(false));
        assert_eq!(eval(&less(val(10), var("y")), &x10).is_err(), true);
    }
    */
}
