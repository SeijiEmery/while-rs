#pragma once
#include "ast.hpp"
#include "state.hpp"

template <typename AST, typename State>
struct _Reduce1;

template <typename AST, typename S>
struct Reduce1 {
    typedef _Reduce1<AST,S> Reduced;
    typedef typename Reduced::Result Result;
    typedef typename Reduced::State State;
    static void print () { Result::print(); }
};

template <typename AST>
struct IsReduced;

template <int val>
struct IsReduced<Value<val>> { 
    enum { result = true };
    static void print () { Value<val>::print(); printf(" is terminal"); }
 };

template <bool b>
struct IsReduced<BConst<b>> { 
    enum { result = true }; 
    static void print () { BConst<b>::print(); printf(" is terminal"); }
};

template <>
struct IsReduced<Skip> { 
    enum { result = true }; 
    static void print () { Skip::print(); printf(" is terminal"); }
};

template <typename AST>
struct IsReduced { 
    enum { result = false }; 
    static void print () { AST::print(); printf(" is non-terminal"); }
};

// Terminal cases

template <int val, typename S>
struct _Reduce1<Value<val>, S> {
    typedef Value<val>  Result;
    typedef S           State;
};
template <char var, typename S>
struct _Reduce1<Var<var>, S> {
    typedef Value<GetVar<var,S>::result> Result;
    typedef S State;
};
template <bool b, typename S>
struct _Reduce1<BConst<b>, S> {
    typedef BConst<b> Result;
    typedef S State;
};
template <typename S>
struct _Reduce1<Skip,S> {
    typedef Skip Result;
    typedef S State;
};

// Unary functions

template <typename S>
struct _Reduce1<Not<BConst<true>>, S> {
    typedef BConst<false> Result;
    typedef S State;
};
template <typename S>
struct _Reduce1<Not<BConst<false>>, S> {
    typedef BConst<true> Result;
    typedef S State;
};
template <typename BExpr, typename S>
struct _Reduce1<Not<BExpr>, S> {
    typedef Reduce1<BExpr, S> Reduced;
    typedef Not<typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};

template <char var, int val, typename S>
struct _Reduce1<Assign<var, Value<val>>, S> {
    typedef Skip Result;
    typedef WithVar<var, val, S> State;
};
template <char var, typename AExpr, typename S>
struct _Reduce1<Assign<var, AExpr>, S> {
    typedef Reduce1<AExpr, S> Reduced;
    typedef Assign<var, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};

// Binary functions

template <typename Next, typename S>
struct _Reduce1<Seq<Skip, Next>, S> {
    typedef Reduce1<Next, S> Reduced;
    typedef typename Reduced::Result Result;
    typedef typename Reduced::State State;
};
template <typename First, typename Second, typename S>
struct _Reduce1<Seq<First, Second>, S> {
    typedef Reduce1<First, S> Reduced;
    typedef Seq<typename Reduced::Result, Second> Result;
    typedef typename Reduced::State State;
};

template <int v1, int v2, typename S>
struct _Reduce1<Add<Value<v1>, Value<v2>>, S> {
    typedef Value<v1 + v2> Result;
    typedef S State;
};
template <int v1, typename Right, typename S>
struct _Reduce1<Add<Value<v1>, Right>, S> {
    typedef Reduce1<Right, S> Reduced;
    typedef Add<Value<v1>, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};
template <typename Left, typename Right, typename S>
struct _Reduce1<Add<Left, Right>, S> {
    typedef Reduce1<Left, S> Reduced;
    typedef Add<typename Reduced::Result, Right> Result;
    typedef typename Reduced::State State;
};

template <int v1, int v2, typename S>
struct _Reduce1<Sub<Value<v1>, Value<v2>>, S> {
    typedef Value<v1 - v2> Result;
    typedef S State;
};
template <int v1, typename Right, typename S>
struct _Reduce1<Sub<Value<v1>, Right>, S> {
    typedef Reduce1<Right, S> Reduced;
    typedef Sub<Value<v1>, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};
template <typename Left, typename Right, typename S>
struct _Reduce1<Sub<Left, Right>, S> {
    typedef Reduce1<Left, S> Reduced;
    typedef Sub<typename Reduced::Result, Right> Result;
    typedef typename Reduced::State State;
};

template <int v1, int v2, typename S>
struct _Reduce1<Mul<Value<v1>, Value<v2>>, S> {
    typedef Value<v1 * v2> Result;
    typedef S State;
};
template <int v1, typename Right, typename S>
struct _Reduce1<Mul<Value<v1>, Right>, S> {
    typedef Reduce1<Right, S> Reduced;
    typedef Mul<Value<v1>, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};
template <typename Left, typename Right, typename S>
struct _Reduce1<Mul<Left, Right>, S> {
    typedef Reduce1<Left, S> Reduced;
    typedef Mul<typename Reduced::Result, Right> Result;
    typedef typename Reduced::State State;
};

template <int v1, int v2, typename S>
struct _Reduce1<Equals<Value<v1>, Value<v2>>, S> {
    typedef BConst<v1 == v2> Result;
    typedef S State;
};
template <int v1, typename Right, typename S>
struct _Reduce1<Equals<Value<v1>, Right>, S> {
    typedef Reduce1<Right, S> Reduced;
    typedef Equals<Value<v1>, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};
template <typename Left, typename Right, typename S>
struct _Reduce1<Equals<Left, Right>, S> {
    typedef Reduce1<Left, S> Reduced;
    typedef Equals<typename Reduced::Result, Right> Result;
    typedef typename Reduced::State State;
};

template <int v1, int v2, typename S>
struct _Reduce1<Less<Value<v1>, Value<v2>>, S> {
    typedef BConst<(v1 < v2)> Result;
    typedef S State;
};
template <int v1, typename Right, typename S>
struct _Reduce1<Less<Value<v1>, Right>, S> {
    typedef Reduce1<Right, S> Reduced;
    typedef Less<Value<v1>, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};
template <typename Left, typename Right, typename S>
struct _Reduce1<Less<Left, Right>, S> {
    typedef Reduce1<Left, S> Reduced;
    typedef Less<typename Reduced::Result, Right> Result;
    typedef typename Reduced::State State;
};

// Boolean operators are short circuiting...
template <typename Right, typename S>
struct _Reduce1<And<BConst<false>, Right>, S> {
    typedef BConst<false> Result;
    typedef S State;
};
template <bool result, typename S>
struct _Reduce1<And<BConst<true>, BConst<result>>, S> {
    typedef BConst<result> Result;
    typedef S State;
};
template <typename Right, typename S>
struct _Reduce1<And<BConst<true>, Right>, S> {
    typedef Reduce1<Right, S> Reduced;
    typedef And<BConst<true>, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};
template <typename Left, typename Right, typename S>
struct _Reduce1<And<Left, Right>, S> {
    typedef Reduce1<Left, S> Reduced;
    typedef And<typename Reduced::Result, Right> Result;
    typedef typename Reduced::State State;
};

template <typename Right, typename S>
struct _Reduce1<Or<BConst<true>, Right>, S> {
    typedef BConst<true> Result;
    typedef S State;
};
template <bool result, typename S>
struct _Reduce1<Or<BConst<false>, BConst<result>>, S> {
    typedef BConst<result> Result;
    typedef S State;
};
template <typename Right, typename S>
struct _Reduce1<Or<BConst<false>, Right>, S> {
    typedef Reduce1<Right, S> Reduced;
    typedef Or<BConst<false>, typename Reduced::Result> Result;
    typedef typename Reduced::State State;
};
template <typename Left, typename Right, typename S>
struct _Reduce1<Or<Left, Right>, S> {
    typedef Reduce1<Left, S> Reduced;
    typedef Or<typename Reduced::Result, Right> Result;
    typedef typename Reduced::State State;
};

// If, While
template <typename IfTrue, typename IfFalse, typename S>
struct _Reduce1<If<BConst<true>, IfTrue, IfFalse>, S> {
    typedef IfTrue Result;
    typedef S State;
};
template <typename IfTrue, typename IfFalse, typename S>
struct _Reduce1<If<BConst<false>, IfTrue, IfFalse>, S> {
    typedef IfFalse Result;
    typedef S State;
};
template <typename IfCond, typename IfTrue, typename IfFalse, typename S>
struct _Reduce1<If<IfCond, IfTrue, IfFalse>, S> {
    typedef Reduce1<IfCond, S> Reduced;
    typedef If<typename Reduced::Result, IfTrue, IfFalse> Result;
    typedef typename Reduced::State State;
};

template <typename C0, typename Body, typename S>
struct _Reduce1<While<BConst<false>, C0, Body>, S> {
    typedef Skip Result;
    typedef S State;
};
template <typename C0, typename Body, typename S>
struct _Reduce1<While<BConst<true>, C0, Body>, S> {
    typedef Reduce1<Seq<Body, While<C0, C0, Body>>, S> Reduced;
    typedef typename Reduced::Result Result;
    typedef typename Reduced::State State;
};
template <typename Cond, typename C0, typename Body, typename S>
struct _Reduce1<While<Cond, C0, Body>, S> {
    typedef Reduce1<Cond, S> Reduced;
    typedef While<typename Reduced::Result, C0, Body> Result;
    typedef typename Reduced::State State;
};
