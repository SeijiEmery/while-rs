# while-rs

This is a small-step while interpreter for CMPS 203.

Note that this does not have a parser / interpreter frontend 
(as that was not required for this assignment); correctness is 
instead demonstrated through unittests.

## Build instructions

    git clone https://github.com/seijiemery/while-rs
    cd while-rs
    cargo test
    
## Unittests

Are currently embedded in / at the end of `aexpr.rs`, `bexpr.rs`, and `cmd.rs`

## Project Structure:

Note: this is currently in need of some refactoring.

- `src/lib.rs`: exposes the while-lang library and test suite
- `src/while_lang/ast/mod.rs`: de facto module exports
- `src/while_lang/ast/aexpr.rs`: arithmetic (integer) expressions
- `src/while_lang/ast/bexpr.rs`: boolean expressions and comparisons
- `src/while_lang/ast/cmd.rs`: commands (ie. program statements)
- `src/while_lang/ast/expr.rs`: core traits to implement all of the above
- `src/while_lang/ast/state.rs`: state representation (`State` trait and an implementation, `HashState`)
- `src/while_lang/ast/state_mocks.rs`: state mocks used for unittesting.

### Assignment notes:

I've used three different languages for these assignments.

This and previous assignments are at:
- <https://github.com/seijiemery/while-rs> (HW 4, used rust)
- <https://github.com/seijiemery/while> (HW 2, used haskell)
- <https://github.com/SeijiEmery/Arith/tree/master/cpp> (HW 1, used c++ template metaprogramming)

Note that arith includes a haskell version for reference, but this was not a haskell project, and the haskell code was
just a direct translation of the c++ template code to show how the c++ template code worked (and demonstrate how the c++ 
template system is effectively a pure functional programming language with pattern matching that operates on c++ types 
and integers at compile time)
