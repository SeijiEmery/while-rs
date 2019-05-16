#include "../src/state.hpp"
#include "../src/macros.hpp"

int main () {
    #undef show
    #define show(expr) \
        printf(#expr " => "); println(expr)

    show(empty_state);
    using a = with_var('x', 10);
    using b = set_var('y', 12, a);
    using c = set_var('x', 11, b);
    printf("\n");

    show(a);
    show(get_var('x', a));
    // show(get_var('y', a));   // uncomment => compile error (undefined variable)
    printf("\n");

    show(b);
    show(get_var('x', b));
    show(get_var('y', b));
    printf("\n");

    show(c);
    show(get_var('x', c));
    show(get_var('y', c));
    // show(get_var('z', c));   // uncomment => compile error (undefined variable)
    printf("\n");

    return 0;
}
