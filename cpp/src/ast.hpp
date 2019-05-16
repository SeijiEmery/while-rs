#pragma once
#include "state.hpp"

// AExpr

template <int val>
struct Value {
    enum { result = val };
    static void print () { printf("%d", result); }
};
template <char variable>
struct Var {
    enum { var = variable };
    static void print () { printf("%c", var); }
};
template <typename L, typename R>
struct Add {
    typedef L ALeft;
    typedef R ARight;
    static void print () { ALeft::print(); printf(" + "); ARight::print(); }
};
template <typename L, typename R>
struct Sub {
    typedef L ALeft;
    typedef R ARight;
    static void print () { ALeft::print(); printf(" - "); ARight::print(); }
};
template <typename L, typename R>
struct Mul {
    typedef L ALeft;
    typedef R ARight;
    static void print () { ALeft::print(); printf(" * "); ARight::print(); }
};

// BExpr

template <bool b>
struct BConst {
    enum { bresult = b };
    static void print () { printf(bresult ? "true" : "false"); }
};
template <typename B>
struct Not {
    typedef B BExpr;
    static void print () { printf("!"); BExpr::print(); }
};
template <typename L, typename R>
struct Equals {
    typedef L ALeft;
    typedef R ARight;
    static void print () { ALeft::print(); printf(" == "); ARight::print(); }
};
template <typename L, typename R>
struct Less {
    typedef L ALeft;
    typedef R ARight;
    static void print () { ALeft::print(); printf(" < "); ARight::print(); }
};
template <typename L, typename R>
struct And {
    typedef L BLeft;
    typedef R BRight;
    static void print () { BLeft::print(); printf(" && "); BRight::print(); }
};
template <typename L, typename R>
struct Or {
    typedef L BLeft;
    typedef R BRight;
    static void print () { BLeft::print(); printf(" || "); BRight::print(); }
};
template <typename L, typename R>
struct Xor {
    typedef L BLeft;
    typedef R BRight;
    static void print () { BLeft::print(); printf(" ^ "); BRight::print(); }
};

// Cmd

struct Skip {
    static void print () { printf("skip"); }
};
template <char variable, typename Expr>
struct Assign {
    enum { var = variable };
    typedef Expr AExpr;
    static void print () { printf("%c := ", var); AExpr::print(); }
};
template <typename C1, typename C2>
struct Seq {
    typedef C1 First;
    typedef C2 Second;
    static void print () { First::print(); printf("; "); Second::print(); }
};
template <typename C, typename T, typename F>
struct If {
    typedef C IfCond;
    typedef T IfTrue;
    typedef F IfFalse;
    static void print () {
        printf("if "); IfCond::print(); printf(" then "); IfTrue::print(); printf(" else "); IfFalse::print();
    }
};
template <typename C, typename C0, typename B>
struct While {
    typedef C WhileCond;
    typedef C0 WhileCond0;
    typedef B WhileBody;
    static void print () {
        printf("while "); WhileCond::print(); printf(" do "); WhileBody::print();
    }
};
