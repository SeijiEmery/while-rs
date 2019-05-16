#pragma once
// #ifndef print
// #define do_print(thing) (thing::print())
// #endif
// #ifndef println
#define println(args...) (args::print(), printf("\n"))
// #endif
