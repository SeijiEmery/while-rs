#![macro_use]
mod while_lang;
use while_lang::*;

#[cfg(test)]
mod tests {
    use super::while_lang::ast::aexpr::tests::*;
    use super::while_lang::ast::bexpr::tests::*;
    use super::while_lang::ast::cmd::tests::*;

    #[test]
    fn stub_for_all_tests () {}
}
