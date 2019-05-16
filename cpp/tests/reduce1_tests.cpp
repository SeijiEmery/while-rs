#include "../src/reduce_1.hpp"
#include "../src/macros.hpp"

template <bool selection, typename L, typename R>
struct Select;

template <typename L, typename R>
struct Select<true, L, R> { typedef L Result; };

template <typename L, typename R>
struct Select<false, L, R> { typedef R Result; };

#define select(cond, left, right...) \
    Select<cond, left, right>::Result

template <typename AST, typename State>
static void print_ast_state () {
    AST::print(); printf(", state = "); State::print();
}
template <typename AST, typename State>
struct DoEvalPrint {
    struct Run {
        static void run () {
            
        }
    };
    struct Terminate { static void run () {} };
    static void run () {
        typedef typename Reduce1<AST, State>::Result ReducedAST;
        typedef typename Reduce1<AST, State>::State NextState;
        printf("=> "); print_ast_state<ReducedAST, NextState>(); printf("\n");
        select(is_reduced(ReducedAST), Terminate, DoEvalPrint<ReducedAST, NextState>)::run();
    }
};
template <typename AST, typename State>
struct EvalPrint {
    static void run () {
        printf("eval("); print_ast_state<AST, State>(); printf(")\n");
        DoEvalPrint<AST, State>::run();
        printf("\n");
    }
};

#undef show
// #define show(ast...) _show(state, ast)
// #define show(ast...) \
//     printf("eval(\n\t"); ast::print(); \
//     printf("\n\t, state = "); state::print(); \
//     printf("\n) => \n\t"); reduce1(state, ast)::Result::print(); \
//     printf("\n\t, state = "); reduce1(state, ast)::State::print(); \
//     printf("\n\n");
#define show(ast...) \
        EvalPrint<ast, state>::run()

template <int a, int b>
void run_gcd () {
    using state = set_var('a', a, with_var('b', b));
    using gcd_algorithm = seq(
        while(bnot(equals(var('a'), var('b'))),
            if(less(var('b'), var('a')),
                assign('a', sub(var('a'), var('b'))),
                assign('b', sub(var('b'), var('a')))))
        , var('a'));
    EvalPrint<gcd_algorithm, state>::run();
}

int main () {
    using state = set_var('x', 1, with_var('y', 2));
    show(val(10));
    show(var('x'));
    show(add(val(10), var('x')));
    show(sub(add(val(2),val(2)), mul(var('x'), var('y'))));
    show(btrue);
    show(bfalse);
    show(equals(val(10), var('x')));
    show(less(val(10), var('x')));
    show(band(btrue, bfalse));
    show(bor(btrue, bfalse));
    // show(bxor(btrue, bfalse));
    show(skip);
    show(assign('x', val(10)));
    show(seq(assign('y', val(10)), assign('y', val(12))));
    show(if(btrue, assign('x', val(10)), assign('y', val(12))));
    show(while(bfalse, skip));

    run_gcd<110, 15>();
    return 0;
}
