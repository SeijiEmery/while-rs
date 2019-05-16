#include "../src/ast.hpp"
#include "../src/macros.hpp"

int main () {
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
    show(bxor(btrue, bfalse));
    show(bnot(btrue));
    show(bnot(bfalse));
    show(bnot(bnot(btrue)));
    show(skip);
    show(assign('x', val(10)));
    show(seq(assign('y', val(10)), assign('y', val(12))));
    show(if(btrue, assign('x', val(10)), assign('y', val(12))));
    show(while(bfalse, skip));
    return 0;
}
