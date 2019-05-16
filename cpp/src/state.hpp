#pragma once
#include <cstdio>

/// class State where
///     empty_state :: State
///     set_var :: Variable -> Value -> State -> State
///     get_var :: Variable -> State -> { result :: Value }
///     print :: IO ()
/// 
/// type Variable = char
/// type Value = int
///
#define empty_state EmptyState
#define with_var(var, val) set_var(var, val, empty_state)
#define set_var(var, val, next) WithVar<var, val, next>
#define get_var(var, state) GetVar<var, state>

/// EmptyState :: State = undefined (sort of...)
struct EmptyState {
    static void print () { printf("[]"); }

    // note:
    //  - printing empty state is a defined operation
    //  - accessing an undefined variable is not
    //
    // As such the former has been defined and the latter has been intentionally
    // left undefined for an EmptyState (GetVar<> expects a WithVar structure
    // and will be missing val, var fields), which means that you'll actually get
    // a compiler error if you try to use an undefined variable in a while program :)
    // 
    // Albeit, the error message will probably be horrible and not particularly helpful
    // unless you're used to the "c++ template metaprograms signal semantic user errors 
    // by cryptic compilation errors" thing...
};

template <typename State>
struct StateHelper;

/// WithVar :: Variable -> Value -> State -> State
template <char variable, int value, typename NextState>
struct WithVar {
    enum { var = variable, val = value };
    typedef NextState Next;

    static void print () {
        printf("[ %c = %d", var, val); StateHelper<Next>::print_elems(); printf(" ]");
    }
};

template <>
struct StateHelper<EmptyState> { static void print_elems () {} };

template <typename State>
struct StateHelper {
    static void print_elems () {
        printf(", %c = %d", State::var, State::val);
        StateHelper<typename State::Next>::print_elems();
    }
};

/// GetVar :: Variable -> State -> { result :: Value }
template <char variable, typename State>
struct GetVar;

template <char var>
struct GetVar<var, EmptyState> {
    static void print () {
        printf("undefined (no such variable '%c')", var);
    }
};

template <char var, int val, typename NextState>
struct GetVar<var, WithVar<var, val, NextState>> {
    enum { result = val };
    static void print () { printf("%d", result); }
};

template <char var, char v2, int val, typename NextState>
struct GetVar<var, WithVar<v2, val, NextState>> {
    typedef GetVar<var, NextState> Result;
    enum { result = Result::result };
    static void print () { Result::print(); }
};
