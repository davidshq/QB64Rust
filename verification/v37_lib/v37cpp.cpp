// C++ library without a header for the verification programs v37_*: built into v37cpp.a by
// build_libs.py. Its names are mangled, so the old compiler's nm step finds them with parameters.
#include <stdint.h>

int cpp_add(int a, int b) { return a + b; }
void cpp_inc(int *p) { *p += 1; }
double cpp_half(double v) { return v / 2; }
static int cpp_count = 0;
void cpp_tick() { cpp_count++; }
int cpp_ticks() { return cpp_count; }
