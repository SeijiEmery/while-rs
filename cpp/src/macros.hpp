#pragma once

#define val(a) Value<a>
#define var(x) Var<x>
#define btrue BConst<1>
#define bfalse BConst<0>
#define skip Skip
#define add(a1,a2) Add<a1,a2>
#define sub(a1,a2) Sub<a1,a2>
#define mul(a1,a2) Mul<a1,a2>
#define assign(x,a) Assign<x,a>
#define equals(a1,a2) Equals<a1,a2>
#define less(a1,a2) Less<a1,a2>
#define band(b1,b2) And<b1,b2>
#define bor(b1,b2) Or<b1,b2>
#define bxor(b1,b2) Xor<b1,b2>
#define bnot(b) Not<b>
#define seq(c1,c2) Seq<c1,c2>
#define if(b1,c1,c2) If<b1,c1,c2>
#define while(b,c) While<b,b,c>

#define println(args...) (args::print(), printf("\n"))
#define show(expr) (printf(#expr " => "), println(expr))

#define reduce1(state,ast...) Reduce1<ast,state>
#define print_eval_1(expr,state) ( \
    printf("reduce1("), \
    expr::print(), \
    print(", "), \
    state::print(), \
    printf(") => "), \
    reduce1(expr,state)::print())
#define is_reduced(ast...) IsReduced<ast>::result
