#[allow(unused_imports)]
pub mod while_lang;
pub use while_lang::*;

#[cfg(test)]
mod tests {
    // Import (and hence run) all tests
    // [allow(unused_imports)] is so that importing these won't cause tons of warnings
    // when building in non-testing mode...
    #[allow(unused_imports)]
    use super::while_lang::ast::aexpr::tests::*;
    #[allow(unused_imports)]
    use super::while_lang::ast::bexpr::tests::*;
    #[allow(unused_imports)]
    use super::while_lang::ast::cmd::tests::*;

    // Stub test so this gets recognized as a test module (IDE support)
    #[test]
    fn stub_all_tests () {}
}
