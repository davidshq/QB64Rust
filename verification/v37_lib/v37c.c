// C library without a header for the verification programs v37_*: built into v37c.a, v37s.a and
// v37o.o by build_libs.py, linked by DECLARE LIBRARY / DECLARE STATIC LIBRARY.
#include <stdint.h>
#include <string.h>

int32_t c_add(int32_t a, int32_t b) { return a + b; }
void c_inc(int32_t *p) { *p += 1; }
int32_t c_len(const char *s) { return (int32_t)strlen(s); }
double c_half(double v) { return v / 2; }
float c_halfs(float v) { return v / 2; }
int64_t c_big(int64_t v) { return v * 3; }
const char *c_hello(void) { return "c-hello"; }
static int32_t c_count = 0;
void c_tick(void) { c_count++; }
int32_t c_ticks(void) { return c_count; }
