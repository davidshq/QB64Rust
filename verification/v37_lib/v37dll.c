// DLL for the verification programs v37_*: built into v37dll.dll by build_libs.py, loaded by
// DECLARE DYNAMIC LIBRARY "v37_lib/v37dll".
#include <stdint.h>
#include <string.h>

#ifdef _WIN32
#    define EXPORT __declspec(dllexport)
#else
#    define EXPORT
#endif

EXPORT int32_t d_add(int32_t a, int32_t b) { return a + b; }
EXPORT void d_inc(int32_t *p) { *p += 1; }
EXPORT int32_t d_len(const char *s) { return (int32_t)strlen(s); }
EXPORT double d_half(double v) { return v / 2; }
EXPORT const char *d_hello(void) { return "d-hello"; }
static int32_t d_count = 0;
EXPORT void d_tick(void) { d_count++; }
EXPORT int32_t d_ticks(void) { return d_count; }
